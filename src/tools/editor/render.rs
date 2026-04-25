
use crossterm::{
    terminal::{self, Clear, ClearType},
    cursor::MoveTo,
    style::{Color as CrosstermColor, SetBackgroundColor, SetForegroundColor, ResetColor},
    queue,
};
use std::io::{self, Write};
use crate::tools::editor::buffer::TextBuffer;
use crate::tools::editor::cursor::{Cursor, CursorSet, Mode, LineNumberMode};
use crate::tools::editor::theme::{Theme, SyntaxHighlighter, HighlightType};
use crate::tools::editor::config::EditorConfig;

#[derive(Debug, Clone, Copy)]
pub struct TerminalSize {
    pub width: u16,
    pub height: u16,
}

pub struct TerminalRenderer {
    size: TerminalSize,
    theme: Theme,
    scroll_offset: usize,
}

impl TerminalRenderer {
    pub fn new(theme: Theme) -> Self {
        let size = Self::get_terminal_size();
        Self {
            size,
            theme,
            scroll_offset: 0,
        }
    }
    
    fn get_terminal_size() -> TerminalSize {
        if let Ok((width, height)) = terminal::size() {
            TerminalSize { width, height }
        } else {
            TerminalSize { width: 80, height: 24 }
        }
    }
    
    pub fn resize(&mut self) {
        self.size = Self::get_terminal_size();
    }
    
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }
    
    pub fn theme(&self) -> &Theme {
        &self.theme
    }
    
    fn calculate_scroll_offset(&mut self, buffer: &TextBuffer, cursor: &Cursor, config: &EditorConfig) {
        let viewport_height = (self.size.height as usize).saturating_sub(2);
        let cursor_line = cursor.line;
        let scrolloff = config.editor.scrolloff;
        
        if cursor_line < self.scroll_offset + scrolloff {
            self.scroll_offset = cursor_line.saturating_sub(scrolloff);
        } else if cursor_line >= self.scroll_offset + viewport_height - scrolloff {
            self.scroll_offset = (cursor_line + scrolloff + 1).saturating_sub(viewport_height);
        }
        
        self.scroll_offset = self.scroll_offset.min(buffer.line_count().saturating_sub(viewport_height));
    }
    
    pub fn render(
        &mut self,
        stdout: &mut impl Write,
        buffer: &TextBuffer,
        cursors: &CursorSet,
        mode: Mode,
        config: &EditorConfig,
        command_line: &str,
    ) -> io::Result<()> {
        let highlighter = SyntaxHighlighter::new_with_theme(self.theme.clone());
        
        self.calculate_scroll_offset(buffer, cursors.primary(), config);
        
        queue!(stdout, Clear(ClearType::All))?;
        
        self.render_editor_area(stdout, buffer, cursors, &highlighter, config)?;
        
        self.render_status_line(stdout, buffer, mode)?;
        
        if mode == Mode::Command || mode == Mode::Palette {
            self.render_command_line(stdout, command_line)?;
        }
        
        let cursor = cursors.primary();
        let cursor_x = self.cursor_screen_x(cursor, buffer)?;
        let cursor_y = self.cursor_screen_y(cursor)?;
        queue!(stdout, MoveTo(cursor_x, cursor_y))?;
        
        stdout.flush()?;
        
        Ok(())
    }
    
    fn render_editor_area(
        &mut self,
        stdout: &mut impl Write,
        buffer: &TextBuffer,
        cursors: &CursorSet,
        highlighter: &SyntaxHighlighter,
        config: &EditorConfig,
    ) -> io::Result<()> {
        let viewport_height = (self.size.height as usize).saturating_sub(2);
        let line_number_width = buffer.line_count().to_string().len().max(3);
        
        for i in 0..viewport_height {
            let line_idx = self.scroll_offset + i;
            
            queue!(stdout, MoveTo(0, i as u16))?;
            
            if line_idx >= buffer.line_count() {
                self.render_empty_line(stdout, line_number_width, '~')?;
                continue;
            }
            
            self.render_line_number(stdout, line_idx + 1, cursors.primary().line, line_number_width, &config.editor.line_numbers)?;
            
            let line_content = buffer.line(line_idx);
            let line_offset = buffer.offset_of_position(line_idx, 0);
            
            let highlights = highlighter.highlight_line(&line_content, line_offset);
            
            self.render_highlighted_line(stdout, &line_content, &highlights, cursors, line_idx, line_offset)?;
        }
        
        Ok(())
    }
    
    fn render_empty_line(&self, stdout: &mut impl Write, width: usize, fill_char: char) -> io::Result<()> {
        self.set_foreground_color(stdout, &self.theme.line_number)?;
        write!(stdout, "{: >width$} ", fill_char, width = width)?;
        self.reset_colors(stdout)?;
        Ok(())
    }
    
    fn render_line_number(
        &self,
        stdout: &mut impl Write,
        line_num: usize,
        cursor_line: usize,
        width: usize,
        mode: &LineNumberMode,
    ) -> io::Result<()> {
        self.set_foreground_color(stdout, &self.theme.line_number)?;
        
        match mode {
            LineNumberMode::None => {}
            LineNumberMode::Absolute => {
                write!(stdout, "{: >width$} ", line_num, width = width)?;
            }
            LineNumberMode::Relative => {
                let relative = if line_num == cursor_line + 1 {
                    line_num
                } else {
                    (line_num as isize - (cursor_line + 1) as isize).abs() as usize
                };
                write!(stdout, "{: >width$} ", relative, width = width)?;
            }
            LineNumberMode::Hybrid => {
                if line_num == cursor_line + 1 {
                    write!(stdout, "{: >width$} ", line_num, width = width)?;
                } else {
                    let relative = (line_num as isize - (cursor_line + 1) as isize).abs() as usize;
                    write!(stdout, "{: >width$} ", relative, width = width)?;
                }
            }
        }
        
        self.reset_colors(stdout)?;
        Ok(())
    }
    
    fn render_highlighted_line(
        &self,
        stdout: &mut impl Write,
        line: &str,
        highlights: &[crate::tools::editor::theme::HighlightSpan],
        cursors: &CursorSet,
        _line_idx: usize,
        line_start_offset: usize,
    ) -> io::Result<()> {
        let chars: Vec<char> = line.chars().collect();
        let mut current_offset = 0;
        let mut highlight_idx = 0;
        
        while current_offset < chars.len() {
            let char_offset = line_start_offset + current_offset;
            
            let mut in_selection = false;
            for cursor in cursors.all() {
                if let Some((sel_start, sel_end)) = cursor.selection_range() {
                    if char_offset >= sel_start && char_offset < sel_end {
                        in_selection = true;
                        break;
                    }
                }
            }
            
            if in_selection {
                self.set_background_color(stdout, &self.theme.selection)?;
            } else {
                self.set_background_color(stdout, &self.theme.background)?;
            }
            
            let mut highlight_type = HighlightType::None;
            while highlight_idx < highlights.len() {
                let hl = &highlights[highlight_idx];
                if char_offset >= hl.start_offset && char_offset < hl.end_offset {
                    highlight_type = hl.highlight_type;
                    break;
                } else if char_offset >= hl.end_offset {
                    highlight_idx += 1;
                } else {
                    break;
                }
            }
            
            self.set_highlight_color(stdout, highlight_type)?;
            
            let c = chars[current_offset];
            if c != '\n' && c != '\r' {
                write!(stdout, "{}", c)?;
            }
            
            current_offset += 1;
        }
        
        self.reset_colors(stdout)?;
        Ok(())
    }
    
    fn set_highlight_color(&self, stdout: &mut impl Write, highlight_type: HighlightType) -> io::Result<()> {
        match highlight_type {
            HighlightType::Keyword => self.set_foreground_color(stdout, &self.theme.keyword)?,
            HighlightType::Type => self.set_foreground_color(stdout, &self.theme.type_name)?,
            HighlightType::Function => self.set_foreground_color(stdout, &self.theme.function)?,
            HighlightType::String => self.set_foreground_color(stdout, &self.theme.string)?,
            HighlightType::Comment => self.set_foreground_color(stdout, &self.theme.comment)?,
            HighlightType::Number => self.set_foreground_color(stdout, &self.theme.number)?,
            HighlightType::Operator => self.set_foreground_color(stdout, &self.theme.operator)?,
            HighlightType::None => self.set_foreground_color(stdout, &self.theme.foreground)?,
        }
        Ok(())
    }
    
    fn render_status_line(
        &self,
        stdout: &mut impl Write,
        buffer: &TextBuffer,
        mode: Mode,
    ) -> io::Result<()> {
        let line = self.size.height - 2;
        
        queue!(stdout, MoveTo(0, line))?;
        
        self.set_background_color(stdout, &self.theme.status_line_bg)?;
        self.set_foreground_color(stdout, &self.theme.status_line_fg)?;
        
        let mode_str = match mode {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
            Mode::VisualLine => "VISUAL LINE",
            Mode::Command => "COMMAND",
            Mode::Palette => "PALETTE",
        };
        
        let filename = buffer.path()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("[无标题]");
        
        let modified = if buffer.is_modified() { " [+]" } else { "" };
        
        let status_line = format!(" {} | {}{}", mode_str, filename, modified);
        
        let padding = " ".repeat((self.size.width as usize).saturating_sub(status_line.len()));
        
        write!(stdout, "{}{}", status_line, padding)?;
        
        self.reset_colors(stdout)?;
        
        Ok(())
    }
    
    fn render_command_line(&self, stdout: &mut impl Write, content: &str) -> io::Result<()> {
        let line = self.size.height - 1;
        
        queue!(stdout, MoveTo(0, line))?;
        
        self.set_background_color(stdout, &self.theme.background)?;
        self.set_foreground_color(stdout, &self.theme.foreground)?;
        
        write!(stdout, ":{}", content)?;
        
        self.reset_colors(stdout)?;
        
        Ok(())
    }
    
    fn set_foreground_color(&self, stdout: &mut impl Write, color: &crate::tools::editor::theme::Color) -> io::Result<()> {
        queue!(stdout, SetForegroundColor(CrosstermColor::Rgb { r: color.r, g: color.g, b: color.b }))
    }
    
    fn set_background_color(&self, stdout: &mut impl Write, color: &crate::tools::editor::theme::Color) -> io::Result<()> {
        queue!(stdout, SetBackgroundColor(CrosstermColor::Rgb { r: color.r, g: color.g, b: color.b }))
    }
    
    fn reset_colors(&self, stdout: &mut impl Write) -> io::Result<()> {
        queue!(stdout, ResetColor)
    }
    
    fn cursor_screen_x(&self, cursor: &Cursor, buffer: &TextBuffer) -> io::Result<u16> {
        let line_number_width = buffer.line_count().to_string().len().max(3);
        Ok((cursor.column + line_number_width + 1) as u16)
    }
    
    fn cursor_screen_y(&self, cursor: &Cursor) -> io::Result<u16> {
        Ok((cursor.line - self.scroll_offset) as u16)
    }
}

impl Default for TerminalRenderer {
    fn default() -> Self {
        let size = TerminalRenderer::get_terminal_size();
        Self {
            size,
            theme: Theme::default(),
            scroll_offset: 0,
        }
    }
}

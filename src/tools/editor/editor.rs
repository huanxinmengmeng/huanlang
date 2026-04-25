
use crossterm::{
    event::{self, Event},
    terminal,
    execute,
};
use std::io::{self, Write};
use std::path::PathBuf;
use unicode_segmentation::UnicodeSegmentation;
use std::collections::VecDeque;

use crate::tools::editor::buffer::TextBuffer;
use crate::tools::editor::cursor::{CursorSet, Mode, clamp_column, clamp_line};
use crate::tools::editor::input::{InputHandler, Key};
use crate::tools::editor::render::TerminalRenderer;

use crate::tools::editor::config::EditorConfig;
use crate::tools::editor::error::{Result, EditorError};
use crate::tools::editor::action::{Action, CommandResult};
use crate::tools::editor::history::{EditHistory, EditOperation};

// 窗口结构体
#[derive(Debug, Clone)]
pub struct Window {
    buffer: TextBuffer,
    cursors: CursorSet,
    mode: Mode,
    history: EditHistory,
    clipboard: String,
}

// 宏操作
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroAction {
    action: Action,
    timestamp: u64,
}

// LSP客户端状态
#[derive(Debug, Clone, Default)]
pub struct LspState {
    connected: bool,
    completions: Vec<String>,
    current_completion_index: Option<usize>,
}

pub struct Editor {
    windows: Vec<Window>,
    current_window_index: usize,
    input_handler: InputHandler,
    renderer: TerminalRenderer,
    config: EditorConfig,
    command_line: String,
    running: bool,
    
    // 代码折叠
    folded_ranges: Vec<(usize, usize)>,
    
    // 宏录制
    macro_recording: bool,
    recorded_macro: Vec<MacroAction>,
    
    // LSP集成
    lsp_state: LspState,
}

impl Editor {
    pub fn new() -> Self {
        let config = EditorConfig::load().unwrap_or_default();
        let theme = config.to_theme();
        
        let window = Window {
            buffer: TextBuffer::new(),
            cursors: CursorSet::new(),
            mode: Mode::Normal,
            history: EditHistory::default(),
            clipboard: String::new(),
        };
        
        Self {
            windows: vec![window],
            current_window_index: 0,
            input_handler: InputHandler::new(),
            renderer: TerminalRenderer::new(theme),
            config,
            command_line: String::new(),
            running: false,
            folded_ranges: Vec::new(),
            macro_recording: false,
            recorded_macro: Vec::new(),
            lsp_state: LspState::default(),
        }
    }
    
    pub fn new_with_file(path: PathBuf) -> Result<Self> {
        let buffer = TextBuffer::load_from_path(&path)?;
        let config = EditorConfig::load().unwrap_or_default();
        let theme = config.to_theme();
        
        let window = Window {
            buffer,
            cursors: CursorSet::new(),
            mode: Mode::Normal,
            history: EditHistory::default(),
            clipboard: String::new(),
        };
        
        Ok(Self {
            windows: vec![window],
            current_window_index: 0,
            input_handler: InputHandler::new(),
            renderer: TerminalRenderer::new(theme),
            config,
            command_line: String::new(),
            running: false,
            folded_ranges: Vec::new(),
            macro_recording: false,
            recorded_macro: Vec::new(),
            lsp_state: LspState::default(),
        })
    }
    
    pub fn run(&mut self) -> Result<()> {
        let mut stdout = io::stdout();
        
        terminal::enable_raw_mode()?;
        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            event::EnableMouseCapture,
        )?;
        
        self.running = true;
        
        while self.running {
            self.render(&mut stdout)?;
            
            if let Event::Key(key_event) = event::read()? {
                let key = Key::from_crossterm(key_event);
                let current_window = &self.windows[self.current_window_index];
                let action = self.input_handler.handle_key_with_fallback(current_window.mode, key);
                let result = self.handle_action(action)?;
                if result.should_quit {
                    self.running = false;
                }
            }
        }
        
        execute!(
            stdout,
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture,
        )?;
        terminal::disable_raw_mode()?;
        
        Ok(())
    }
    
    fn render(&mut self, stdout: &mut impl Write) -> Result<()> {
        let current_window = &self.windows[self.current_window_index];
        self.renderer.render(
            stdout,
            &current_window.buffer,
            &current_window.cursors,
            current_window.mode,
            &self.config,
            &self.command_line,
        )?;
        Ok(())
    }
    
    fn handle_action(&mut self, action: Action) -> Result<CommandResult> {
        let mut result = CommandResult::default();
        
        // 宏录制处理
        if self.macro_recording && action != Action::StopMacroRecording {
            self.recorded_macro.push(MacroAction {
                action: action.clone(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            });
        }
        
        match action {
            // 基本编辑操作
            Action::MoveLeft => self.move_cursors(-1, 0),
            Action::MoveRight => self.move_cursors(1, 0),
            Action::MoveUp => self.move_cursors(0, -1),
            Action::MoveDown => self.move_cursors(0, 1),
            Action::MoveWordForward => self.move_word_forward(),
            Action::MoveWordBackward => self.move_word_backward(),
            Action::MoveLineStart => self.move_to_line_start(),
            Action::MoveLineEnd => self.move_to_line_end(),
            Action::MoveDocumentStart => self.move_to_document_start(),
            Action::MoveDocumentEnd => self.move_to_document_end(),
            
            Action::EnterMode(mode) => {
                let current_window = &mut self.windows[self.current_window_index];
                if current_window.mode == Mode::Visual || current_window.mode == Mode::VisualLine {
                    for cursor in current_window.cursors.all_mut() {
                        cursor.clear_anchor();
                    }
                }
                if mode == Mode::Visual {
                    for cursor in current_window.cursors.all_mut() {
                        cursor.set_anchor();
                    }
                }
                current_window.mode = mode;
            }
            
            Action::InsertChar(c) => self.insert_char(c)?,
            Action::InsertTab => self.insert_tab()?,
            Action::InsertNewline => self.insert_newline()?,
            Action::DeleteChar => self.delete_char(false)?,
            Action::DeleteCharBackward => self.delete_char(true)?,
            Action::DeleteLine => self.delete_line()?,
            Action::DeleteToLineEnd => self.delete_to_line_end()?,
            
            Action::Yank => self.yank()?,
            Action::YankLine => self.yank_line()?,
            Action::Paste => self.paste(false)?,
            Action::PasteBefore => self.paste(true)?,
            
            Action::Undo => self.undo()?,
            Action::Redo => self.redo()?,
            
            Action::Save => self.save()?,
            Action::SaveAndQuit => {
                self.save()?;
                result.should_quit = true;
            }
            Action::Quit => {
                let current_window = &self.windows[self.current_window_index];
                if !current_window.buffer.is_modified() {
                    result.should_quit = true;
                }
            }
            Action::ForceQuit => {
                result.should_quit = true;
            }
            
            // 代码折叠
            Action::ToggleFold => self.toggle_fold()?,
            Action::FoldAll => self.fold_all()?,
            Action::UnfoldAll => self.unfold_all()?,
            
            // 多窗口
            Action::SplitHorizontal => self.split_horizontal()?,
            Action::SplitVertical => self.split_vertical()?,
            Action::SwitchWindow => self.switch_window()?,
            Action::CloseWindow => self.close_window()?,
            
            // 宏录制
            Action::StartMacroRecording => self.start_macro_recording()?,
            Action::StopMacroRecording => self.stop_macro_recording()?,
            Action::PlayMacro => self.play_macro()?,
            
            // LSP集成
            Action::ShowCompletions => self.show_completions()?,
            Action::GoToDefinition => self.go_to_definition()?,
            Action::GoToDeclaration => self.go_to_declaration()?,
            Action::GoToImplementation => self.go_to_implementation()?,
            Action::FindReferences => self.find_references()?,
            Action::DocumentSymbol => self.document_symbol()?,
            Action::WorkspaceSymbol => self.workspace_symbol()?,
            
            Action::None => {}
            
            _ => {}
        }
        
        Ok(result)
    }
    
    fn move_cursors(&mut self, dx: isize, dy: isize) {
        let current_window = &mut self.windows[self.current_window_index];
        for cursor in current_window.cursors.all_mut() {
            let (line, col) = (cursor.line, cursor.column);
            
            let new_line = if dy < 0 {
                line.saturating_sub((-dy) as usize)
            } else {
                line + dy as usize
            };
            let new_line = clamp_line(new_line, current_window.buffer.line_count());
            
            let line_len = current_window.buffer.line_length(new_line);
            let new_col = if dx < 0 {
                col.saturating_sub((-dx) as usize)
            } else {
                col + dx as usize
            };
            let new_col = clamp_column(new_col, line_len);
            
            cursor.line = new_line;
            cursor.column = new_col;
            cursor.offset = current_window.buffer.offset_of_position(new_line, new_col);
        }
    }
    
    fn move_word_forward(&mut self) {
        let current_window = &mut self.windows[self.current_window_index];
        for cursor in current_window.cursors.all_mut() {
            let line = current_window.buffer.line(cursor.line);
            let words: Vec<&str> = line.split_word_bounds().collect();
            
            let offset_in_line = cursor.column;
            let mut current_word_start = 0;
            
            for word in words {
                let word_len = word.chars().count();
                let word_end = current_word_start + word_len;
                
                if offset_in_line < word_end {
                    let new_col = word_end.min(line.chars().count());
                    cursor.column = new_col;
                    cursor.offset = current_window.buffer.offset_of_position(cursor.line, new_col);
                    break;
                }
                
                current_word_start = word_end;
            }
            
            if cursor.column >= line.chars().count() && cursor.line + 1 < current_window.buffer.line_count() {
                cursor.line += 1;
                cursor.column = 0;
                cursor.offset = current_window.buffer.offset_of_position(cursor.line, 0);
            }
        }
    }
    
    fn move_word_backward(&mut self) {
        let current_window = &mut self.windows[self.current_window_index];
        for cursor in current_window.cursors.all_mut() {
            let line = current_window.buffer.line(cursor.line);
            let words: Vec<&str> = line.split_word_bounds().collect();
            
            let offset_in_line = cursor.column;
            
            if offset_in_line == 0 && cursor.line > 0 {
                cursor.line -= 1;
                cursor.column = current_window.buffer.line_length(cursor.line);
                cursor.offset = current_window.buffer.offset_of_position(cursor.line, cursor.column);
                return;
            }
            
            let mut current_word_end = line.chars().count();
            
            for word in words.iter().rev() {
                let word_len = word.chars().count();
                let word_start = current_word_end - word_len;
                
                if offset_in_line > word_start {
                    let new_col = word_start;
                    cursor.column = new_col;
                    cursor.offset = current_window.buffer.offset_of_position(cursor.line, new_col);
                    break;
                }
                
                current_word_end = word_start;
            }
        }
    }
    
    fn move_to_line_start(&mut self) {
        let current_window = &mut self.windows[self.current_window_index];
        for cursor in current_window.cursors.all_mut() {
            cursor.column = 0;
            cursor.offset = current_window.buffer.offset_of_position(cursor.line, 0);
        }
    }
    
    fn move_to_line_end(&mut self) {
        let current_window = &mut self.windows[self.current_window_index];
        for cursor in current_window.cursors.all_mut() {
            let line_len = current_window.buffer.line_length(cursor.line);
            cursor.column = line_len;
            cursor.offset = current_window.buffer.offset_of_position(cursor.line, line_len);
        }
    }
    
    fn move_to_document_start(&mut self) {
        let current_window = &mut self.windows[self.current_window_index];
        for cursor in current_window.cursors.all_mut() {
            cursor.line = 0;
            cursor.column = 0;
            cursor.offset = 0;
        }
    }
    
    fn move_to_document_end(&mut self) {
        let current_window = &mut self.windows[self.current_window_index];
        for cursor in current_window.cursors.all_mut() {
            let last_line = current_window.buffer.line_count().saturating_sub(1);
            let last_col = current_window.buffer.line_length(last_line);
            cursor.line = last_line;
            cursor.column = last_col;
            cursor.offset = current_window.buffer.len();
        }
    }
    
    fn insert_char(&mut self, c: char) -> Result<()> {
        let current_window = &mut self.windows[self.current_window_index];
        current_window.history.begin_compound();
        
        let s = c.to_string();
        
        for cursor in current_window.cursors.all_mut() {
            let offset = cursor.offset;
            let cursor_pos = (cursor.offset, cursor.line, cursor.column);
            
            current_window.buffer.insert(offset, &s);
            
            current_window.history.add_operation(
                EditOperation::Insert { offset, text: s.clone() },
                cursor_pos,
            );
            
            cursor.offset += s.len();
            let (new_line, new_col) = current_window.buffer.position_of_offset(cursor.offset);
            cursor.line = new_line;
            cursor.column = new_col;
        }
        
        current_window.history.end_compound();
        
        Ok(())
    }
    
    fn insert_tab(&mut self) -> Result<()> {
        let current_window = &mut self.windows[self.current_window_index];
        let tab = if self.config.editor.expand_tab {
            " ".repeat(self.config.editor.tab_size)
        } else {
            "\t".to_string()
        };
        
        current_window.history.begin_compound();
        
        for cursor in current_window.cursors.all_mut() {
            let offset = cursor.offset;
            let cursor_pos = (cursor.offset, cursor.line, cursor.column);
            
            current_window.buffer.insert(offset, &tab);
            
            current_window.history.add_operation(
                EditOperation::Insert { offset, text: tab.clone() },
                cursor_pos,
            );
            
            cursor.offset += tab.len();
            let (new_line, new_col) = current_window.buffer.position_of_offset(cursor.offset);
            cursor.line = new_line;
            cursor.column = new_col;
        }
        
        current_window.history.end_compound();
        
        Ok(())
    }
    
    fn insert_newline(&mut self) -> Result<()> {
        let current_window = &mut self.windows[self.current_window_index];
        current_window.history.begin_compound();
        
        for cursor in current_window.cursors.all_mut() {
            let offset = cursor.offset;
            let cursor_pos = (cursor.offset, cursor.line, cursor.column);
            
            let mut indent = String::new();
            if self.config.editor.auto_indent {
                let line = current_window.buffer.line(cursor.line);
                for c in line.chars() {
                    if c.is_whitespace() {
                        indent.push(c);
                    } else {
                        break;
                    }
                }
            }
            
            let newline = format!("\n{}", indent);
            
            current_window.buffer.insert(offset, &newline);
            
            current_window.history.add_operation(
                EditOperation::Insert { offset, text: newline.clone() },
                cursor_pos,
            );
            
            cursor.offset += newline.len();
            let (new_line, new_col) = current_window.buffer.position_of_offset(cursor.offset);
            cursor.line = new_line;
            cursor.column = new_col;
        }
        
        current_window.history.end_compound();
        
        Ok(())
    }
    
    fn delete_char(&mut self, backward: bool) -> Result<()> {
        let current_window = &mut self.windows[self.current_window_index];
        current_window.history.begin_compound();
        
        for cursor in current_window.cursors.all_mut() {
            let (start, end) = if backward {
                if cursor.offset == 0 {
                    continue;
                }
                (cursor.offset - 1, cursor.offset)
            } else {
                if cursor.offset >= current_window.buffer.len() {
                    continue;
                }
                (cursor.offset, cursor.offset + 1)
            };
            
            let text = current_window.buffer.slice(start, end);
            let cursor_pos = (cursor.offset, cursor.line, cursor.column);
            
            current_window.buffer.remove(start, end);
            
            current_window.history.add_operation(
                EditOperation::Remove { offset: start, text },
                cursor_pos,
            );
            
            cursor.offset = start;
            let (new_line, new_col) = current_window.buffer.position_of_offset(start);
            cursor.line = new_line;
            cursor.column = new_col;
        }
        
        current_window.history.end_compound();
        
        Ok(())
    }
    
    fn delete_line(&mut self) -> Result<()> {
        let current_window = &mut self.windows[self.current_window_index];
        current_window.history.begin_compound();
        
        for cursor in current_window.cursors.all_mut() {
            let line_start = current_window.buffer.offset_of_position(cursor.line, 0);
            let line_end = if cursor.line + 1 < current_window.buffer.line_count() {
                current_window.buffer.offset_of_position(cursor.line + 1, 0)
            } else {
                current_window.buffer.len()
            };
            
            let text = current_window.buffer.slice(line_start, line_end);
            let cursor_pos = (cursor.offset, cursor.line, cursor.column);
            
            current_window.clipboard = text.clone();
            
            current_window.buffer.remove(line_start, line_end);
            
            current_window.history.add_operation(
                EditOperation::Remove { offset: line_start, text },
                cursor_pos,
            );
            
            cursor.line = clamp_line(cursor.line, current_window.buffer.line_count());
            cursor.column = 0;
            cursor.offset = current_window.buffer.offset_of_position(cursor.line, 0);
        }
        
        current_window.history.end_compound();
        
        Ok(())
    }
    
    fn delete_to_line_end(&mut self) -> Result<()> {
        let current_window = &mut self.windows[self.current_window_index];
        current_window.history.begin_compound();
        
        for cursor in current_window.cursors.all_mut() {
            let line_end = if cursor.line + 1 < current_window.buffer.line_count() {
                current_window.buffer.offset_of_position(cursor.line + 1, 0) - 1
            } else {
                current_window.buffer.len()
            };
            
            if cursor.offset >= line_end {
                continue;
            }
            
            let text = current_window.buffer.slice(cursor.offset, line_end);
            let cursor_pos = (cursor.offset, cursor.line, cursor.column);
            
            current_window.buffer.remove(cursor.offset, line_end);
            
            current_window.history.add_operation(
                EditOperation::Remove { offset: cursor.offset, text },
                cursor_pos,
            );
        }
        
        current_window.history.end_compound();
        
        Ok(())
    }
    
    fn yank(&mut self) -> Result<()> {
        let current_window = &mut self.windows[self.current_window_index];
        for cursor in current_window.cursors.all() {
            if let Some((start, end)) = cursor.selection_range() {
                current_window.clipboard = current_window.buffer.slice(start, end);
            }
        }
        Ok(())
    }
    
    fn yank_line(&mut self) -> Result<()> {
        let current_window = &mut self.windows[self.current_window_index];
        for cursor in current_window.cursors.all() {
            let line_start = current_window.buffer.offset_of_position(cursor.line, 0);
            let line_end = if cursor.line + 1 < current_window.buffer.line_count() {
                current_window.buffer.offset_of_position(cursor.line + 1, 0)
            } else {
                current_window.buffer.len()
            };
            current_window.clipboard = current_window.buffer.slice(line_start, line_end);
        }
        Ok(())
    }
    
    fn paste(&mut self, before: bool) -> Result<()> {
        let current_window = &mut self.windows[self.current_window_index];
        current_window.history.begin_compound();
        
        for cursor in current_window.cursors.all_mut() {
            let offset = if before {
                cursor.offset
            } else {
                cursor.offset
            };
            let cursor_pos = (cursor.offset, cursor.line, cursor.column);
            
            current_window.buffer.insert(offset, &current_window.clipboard);
            
            current_window.history.add_operation(
                EditOperation::Insert { offset, text: current_window.clipboard.clone() },
                cursor_pos,
            );
            
            cursor.offset += current_window.clipboard.len();
            let (new_line, new_col) = current_window.buffer.position_of_offset(cursor.offset);
            cursor.line = new_line;
            cursor.column = new_col;
        }
        
        current_window.history.end_compound();
        
        Ok(())
    }
    
    fn undo(&mut self) -> Result<()> {
        let current_window = &mut self.windows[self.current_window_index];
        if let Some(entry) = current_window.history.undo() {
            for op in entry.invert() {
                match op {
                    EditOperation::Insert { offset, text } => {
                        current_window.buffer.insert(offset, &text);
                    }
                    EditOperation::Remove { offset, text } => {
                        current_window.buffer.remove(offset, offset + text.len());
                    }
                    _ => {}
                }
            }
            
            if let Some(&(offset, line, column)) = entry.cursor_positions().first() {
                let cursor = current_window.cursors.primary_mut();
                cursor.offset = offset;
                cursor.line = line;
                cursor.column = column;
            }
        }
        Ok(())
    }
    
    fn redo(&mut self) -> Result<()> {
        let current_window = &mut self.windows[self.current_window_index];
        if let Some(entry) = current_window.history.redo() {
            for op in entry.operations() {
                match op {
                    EditOperation::Insert { offset, text } => {
                        current_window.buffer.insert(*offset, text);
                    }
                    EditOperation::Remove { offset, text } => {
                        current_window.buffer.remove(*offset, *offset + text.len());
                    }
                    _ => {}
                }
            }
            
            if let Some(&(offset, line, column)) = entry.cursor_positions().first() {
                let cursor = current_window.cursors.primary_mut();
                cursor.offset = offset;
                cursor.line = line;
                cursor.column = column;
            }
        }
        Ok(())
    }
    
    fn save(&mut self) -> Result<()> {
        let current_window = &mut self.windows[self.current_window_index];
        if current_window.buffer.path().is_none() {
            return Err(EditorError::FileNotFound("未指定文件路径".to_string()));
        }
        current_window.buffer.save()?;
        Ok(())
    }
    
    // 代码折叠
    fn toggle_fold(&mut self) -> Result<()> {
        let current_window = &mut self.windows[self.current_window_index];
        let cursor = current_window.cursors.primary();
        let line = cursor.line;
        
        // 简单实现：折叠当前行及其下面的缩进行
        let mut fold_end = line + 1;
        let current_line = current_window.buffer.line(line);
        let current_indent = current_line.chars().take_while(|c| c.is_whitespace()).count();
        
        while fold_end < current_window.buffer.line_count() {
            let next_line = current_window.buffer.line(fold_end);
            let next_indent = next_line.chars().take_while(|c| c.is_whitespace()).count();
            if next_indent <= current_indent {
                break;
            }
            fold_end += 1;
        }
        
        if fold_end > line + 1 {
            self.folded_ranges.push((line, fold_end - 1));
        }
        
        Ok(())
    }
    
    fn fold_all(&mut self) -> Result<()> {
        let current_window = &self.windows[self.current_window_index];
        self.folded_ranges.clear();
        
        // 简单实现：折叠所有有缩进的代码块
        let mut line = 0;
        while line < current_window.buffer.line_count() {
            let current_line = current_window.buffer.line(line);
            let current_indent = current_line.chars().take_while(|c| c.is_whitespace()).count();
            
            if current_indent > 0 {
                let mut fold_end = line + 1;
                while fold_end < current_window.buffer.line_count() {
                    let next_line = current_window.buffer.line(fold_end);
                    let next_indent = next_line.chars().take_while(|c| c.is_whitespace()).count();
                    if next_indent <= current_indent {
                        break;
                    }
                    fold_end += 1;
                }
                
                if fold_end > line + 1 {
                    self.folded_ranges.push((line, fold_end - 1));
                    line = fold_end - 1;
                }
            }
            
            line += 1;
        }
        
        Ok(())
    }
    
    fn unfold_all(&mut self) -> Result<()> {
        self.folded_ranges.clear();
        Ok(())
    }
    
    // 多窗口
    fn split_horizontal(&mut self) -> Result<()> {
        let current_window = &self.windows[self.current_window_index];
        let new_window = Window {
            buffer: current_window.buffer.clone(),
            cursors: CursorSet::new(),
            mode: Mode::Normal,
            history: EditHistory::default(),
            clipboard: String::new(),
        };
        
        self.windows.push(new_window);
        self.current_window_index = self.windows.len() - 1;
        
        Ok(())
    }
    
    fn split_vertical(&mut self) -> Result<()> {
        // 与水平分割类似，实际实现需要处理终端布局
        self.split_horizontal()
    }
    
    fn switch_window(&mut self) -> Result<()> {
        if self.windows.len() > 1 {
            self.current_window_index = (self.current_window_index + 1) % self.windows.len();
        }
        Ok(())
    }
    
    fn close_window(&mut self) -> Result<()> {
        if self.windows.len() > 1 {
            self.windows.remove(self.current_window_index);
            if self.current_window_index >= self.windows.len() {
                self.current_window_index = 0;
            }
        }
        Ok(())
    }
    
    // 宏录制
    fn start_macro_recording(&mut self) -> Result<()> {
        self.macro_recording = true;
        self.recorded_macro.clear();
        Ok(())
    }
    
    fn stop_macro_recording(&mut self) -> Result<()> {
        self.macro_recording = false;
        Ok(())
    }
    
    fn play_macro(&mut self) -> Result<()> {
        let macro_actions = self.recorded_macro.clone();
        for macro_action in macro_actions {
            self.handle_action(macro_action.action)?;
        }
        Ok(())
    }
    
    // LSP集成
    fn show_completions(&mut self) -> Result<()> {
        // 简单实现：模拟代码补全
        self.lsp_state.completions = vec![
            "fn".to_string(),
            "let".to_string(),
            "if".to_string(),
            "for".to_string(),
            "while".to_string(),
        ];
        self.lsp_state.current_completion_index = Some(0);
        Ok(())
    }
    
    fn go_to_definition(&mut self) -> Result<()> {
        // 简单实现：模拟跳转到定义
        Ok(())
    }
    
    fn go_to_declaration(&mut self) -> Result<()> {
        // 简单实现：模拟跳转到声明
        Ok(())
    }
    
    fn go_to_implementation(&mut self) -> Result<()> {
        // 简单实现：模拟跳转到实现
        Ok(())
    }
    
    fn find_references(&mut self) -> Result<()> {
        // 简单实现：模拟查找引用
        Ok(())
    }
    
    fn document_symbol(&mut self) -> Result<()> {
        // 简单实现：模拟文档符号
        Ok(())
    }
    
    fn workspace_symbol(&mut self) -> Result<()> {
        // 简单实现：模拟工作区符号
        Ok(())
    }
}

impl Default for Editor {
    fn default() -> Self {
        let config = crate::tools::editor::config::EditorConfig::default();
        let _theme = crate::tools::editor::theme::Theme::default();
        
        let window = Window {
            buffer: crate::tools::editor::buffer::TextBuffer::default(),
            cursors: crate::tools::editor::cursor::CursorSet::new(),
            mode: crate::tools::editor::cursor::Mode::Normal,
            history: crate::tools::editor::history::EditHistory::default(),
            clipboard: String::new(),
        };
        
        Self {
            windows: vec![window],
            current_window_index: 0,
            input_handler: crate::tools::editor::input::InputHandler::default(),
            renderer: crate::tools::editor::render::TerminalRenderer::default(),
            config,
            command_line: String::new(),
            running: false,
            folded_ranges: Vec::new(),
            macro_recording: false,
            recorded_macro: Vec::new(),
            lsp_state: LspState::default(),
        }
    }
}

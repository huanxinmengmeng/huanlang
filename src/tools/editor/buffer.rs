
use ropey::Rope;
use std::borrow::Cow;
use std::path::Path;
use std::fs;
use crate::tools::editor::error::Result;

#[derive(Debug, Clone)]
pub struct TextBuffer {
    rope: Rope,
    path: Option<std::path::PathBuf>,
    modified: bool,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            path: None,
            modified: false,
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        Self {
            rope: Rope::from_str(s),
            path: None,
            modified: false,
        }
    }
    
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(Self {
            rope: Rope::from_str(&content),
            path: Some(path.to_path_buf()),
            modified: false,
        })
    }
    
    pub fn save_to_path(&mut self, path: &Path) -> Result<()> {
        fs::write(path, self.to_string())?;
        self.path = Some(path.to_path_buf());
        self.modified = false;
        Ok(())
    }
    
    pub fn save(&mut self) -> Result<()> {
        if let Some(path) = self.path.clone() {
            self.save_to_path(&path)
        } else {
            Err(crate::tools::editor::error::EditorError::FileNotFound(
                "未指定文件路径".to_string()
            ))
        }
    }
    
    pub fn len(&self) -> usize {
        self.rope.len_chars()
    }
    
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }
    
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }
    
    pub fn line(&self, line_idx: usize) -> Cow<'_, str> {
        if line_idx < self.rope.len_lines() {
            self.rope.line(line_idx).into()
        } else {
            Cow::Borrowed("")
        }
    }
    
    pub fn line_length(&self, line_idx: usize) -> usize {
        if line_idx < self.rope.len_lines() {
            self.rope.line(line_idx).len_chars()
        } else {
            0
        }
    }
    
    pub fn offset_of_position(&self, line: usize, column: usize) -> usize {
        let line_clamped = line.min(self.rope.len_lines().saturating_sub(1));
        let line_start = self.rope.line_to_char(line_clamped);
        let line_len = self.rope.line(line_clamped).len_chars();
        let column_clamped = column.min(line_len);
        line_start + column_clamped
    }
    
    pub fn position_of_offset(&self, offset: usize) -> (usize, usize) {
        let offset_clamped = offset.min(self.rope.len_chars());
        let line = self.rope.char_to_line(offset_clamped);
        let line_start = self.rope.line_to_char(line);
        let column = offset_clamped - line_start;
        (line, column)
    }
    
    pub fn insert(&mut self, offset: usize, text: &str) {
        let offset_clamped = offset.min(self.rope.len_chars());
        self.rope.insert(offset_clamped, text);
        self.modified = true;
    }
    
    pub fn remove(&mut self, start: usize, end: usize) {
        let start_clamped = start.min(self.rope.len_chars());
        let end_clamped = end.min(self.rope.len_chars()).max(start_clamped);
        self.rope.remove(start_clamped..end_clamped);
        self.modified = true;
    }
    
    pub fn replace(&mut self, start: usize, end: usize, text: &str) {
        let start_clamped = start.min(self.rope.len_chars());
        let end_clamped = end.min(self.rope.len_chars()).max(start_clamped);
        self.rope.remove(start_clamped..end_clamped);
        self.rope.insert(start_clamped, text);
        self.modified = true;
    }
    
    pub fn slice(&self, start: usize, end: usize) -> String {
        let start_clamped = start.min(self.rope.len_chars());
        let end_clamped = end.min(self.rope.len_chars()).max(start_clamped);
        self.rope.slice(start_clamped..end_clamped).to_string()
    }
    
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }
    
    pub fn path(&self) -> Option<&std::path::Path> {
        self.path.as_deref()
    }
    
    pub fn set_path(&mut self, path: std::path::PathBuf) {
        self.path = Some(path);
    }
    
    pub fn is_modified(&self) -> bool {
        self.modified
    }
    
    pub fn set_modified(&mut self, modified: bool) {
        self.modified = modified;
    }
    
    pub fn rope(&self) -> &Rope {
        &self.rope
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

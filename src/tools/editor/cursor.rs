
use std::cmp;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    VisualLine,
    Command,
    Palette,
}

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
    pub anchor: Option<usize>,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            offset: 0,
            line: 0,
            column: 0,
            anchor: None,
        }
    }
}

impl Cursor {
    pub fn new(offset: usize, line: usize, column: usize) -> Self {
        Self {
            offset,
            line,
            column,
            anchor: None,
        }
    }
    
    pub fn selection_range(&self) -> Option<(usize, usize)> {
        self.anchor.map(|anchor| {
            if anchor <= self.offset {
                (anchor, self.offset)
            } else {
                (self.offset, anchor)
            }
        })
    }
    
    pub fn set_anchor(&mut self) {
        self.anchor = Some(self.offset);
    }
    
    pub fn clear_anchor(&mut self) {
        self.anchor = None;
    }
}

#[derive(Debug, Clone)]
pub struct CursorSet {
    cursors: Vec<Cursor>,
    primary: usize,
}

impl Default for CursorSet {
    fn default() -> Self {
        Self::new()
    }
}

impl CursorSet {
    pub fn new() -> Self {
        Self {
            cursors: vec![Cursor::default()],
            primary: 0,
        }
    }
    
    pub fn primary(&self) -> &Cursor {
        &self.cursors[self.primary]
    }
    
    pub fn primary_mut(&mut self) -> &mut Cursor {
        &mut self.cursors[self.primary]
    }
    
    pub fn all(&self) -> &[Cursor] {
        &self.cursors
    }
    
    pub fn all_mut(&mut self) -> &mut [Cursor] {
        &mut self.cursors
    }
    
    pub fn add_cursor(&mut self, cursor: Cursor) {
        self.cursors.push(cursor);
        self.sort_and_dedup();
    }
    
    pub fn remove_secondary_cursors(&mut self) {
        self.cursors.truncate(1);
        self.primary = 0;
    }
    
    pub fn sort_and_dedup(&mut self) {
        self.cursors.sort_by_key(|c| c.offset);
        self.cursors.dedup_by_key(|c| c.offset);
        if self.primary >= self.cursors.len() {
            self.primary = 0;
        }
    }
    
    pub fn len(&self) -> usize {
        self.cursors.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.cursors.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineNumberMode {
    None,
    Absolute,
    Relative,
    Hybrid,
}

impl Default for LineNumberMode {
    fn default() -> Self {
        Self::Absolute
    }
}

pub fn clamp_column(column: usize, line_length: usize) -> usize {
    cmp::min(column, line_length)
}

pub fn clamp_line(line: usize, line_count: usize) -> usize {
    cmp::min(line, line_count.saturating_sub(1))
}

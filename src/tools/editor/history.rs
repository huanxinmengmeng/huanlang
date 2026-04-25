
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum EditOperation {
    Insert {
        offset: usize,
        text: String,
    },
    Remove {
        offset: usize,
        text: String,
    },
    Compound {
        operations: Vec<EditOperation>,
    },
}

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    operations: Vec<EditOperation>,
    cursor_positions: Vec<(usize, usize, usize)>,
    _timestamp: std::time::Instant,
}

impl HistoryEntry {
    pub fn new(operations: Vec<EditOperation>, cursor_positions: Vec<(usize, usize, usize)>) -> Self {
        Self {
            operations,
            cursor_positions,
            _timestamp: std::time::Instant::now(),
        }
    }
    
    pub fn invert(&self) -> Vec<EditOperation> {
        let mut inverted = Vec::new();
        for op in self.operations.iter().rev() {
            match op {
                EditOperation::Insert { offset, text } => {
                    inverted.push(EditOperation::Remove {
                        offset: *offset,
                        text: text.clone(),
                    });
                }
                EditOperation::Remove { offset, text } => {
                    inverted.push(EditOperation::Insert {
                        offset: *offset,
                        text: text.clone(),
                    });
                }
                EditOperation::Compound { operations } => {
                    let mut compound_inverted = Vec::new();
                    for sub_op in operations.iter().rev() {
                        match sub_op {
                            EditOperation::Insert { offset, text } => {
                                compound_inverted.push(EditOperation::Remove {
                                    offset: *offset,
                                    text: text.clone(),
                                });
                            }
                            EditOperation::Remove { offset, text } => {
                                compound_inverted.push(EditOperation::Insert {
                                    offset: *offset,
                                    text: text.clone(),
                                });
                            }
                            EditOperation::Compound { .. } => {}
                        }
                    }
                    inverted.push(EditOperation::Compound {
                        operations: compound_inverted,
                    });
                }
            }
        }
        inverted
    }
    
    pub fn operations(&self) -> &Vec<EditOperation> {
        &self.operations
    }
    
    pub fn cursor_positions(&self) -> &Vec<(usize, usize, usize)> {
        &self.cursor_positions
    }
}

#[derive(Debug, Clone)]
pub struct EditHistory {
    undo_stack: VecDeque<HistoryEntry>,
    redo_stack: VecDeque<HistoryEntry>,
    max_history: usize,
    pending_entry: Option<HistoryEntry>,
}

impl Default for EditHistory {
    fn default() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_history: 1000,
            pending_entry: None,
        }
    }
}

impl EditHistory {
    pub fn new(max_history: usize) -> Self {
        Self {
            undo_stack: VecDeque::with_capacity(max_history),
            redo_stack: VecDeque::with_capacity(max_history),
            max_history,
            pending_entry: None,
        }
    }
    
    pub fn begin_compound(&mut self) {
        self.pending_entry = Some(HistoryEntry::new(Vec::new(), Vec::new()));
    }
    
    pub fn add_operation(&mut self, operation: EditOperation, cursor_position: (usize, usize, usize)) {
        if let Some(ref mut entry) = self.pending_entry {
            entry.operations.push(operation);
            entry.cursor_positions.push(cursor_position);
        } else {
            let entry = HistoryEntry::new(vec![operation], vec![cursor_position]);
            self.push_undo(entry);
        }
    }
    
    pub fn end_compound(&mut self) {
        if let Some(entry) = self.pending_entry.take() {
            if !entry.operations.is_empty() {
                self.push_undo(entry);
            }
        }
    }
    
    fn push_undo(&mut self, entry: HistoryEntry) {
        self.undo_stack.push_back(entry);
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.pop_front();
        }
        self.redo_stack.clear();
    }
    
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }
    
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
    
    pub fn undo(&mut self) -> Option<HistoryEntry> {
        let entry = self.undo_stack.pop_back()?;
        self.redo_stack.push_back(entry.clone());
        Some(entry)
    }
    
    pub fn redo(&mut self) -> Option<HistoryEntry> {
        let entry = self.redo_stack.pop_back()?;
        self.undo_stack.push_back(entry.clone());
        Some(entry)
    }
    
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.pending_entry = None;
    }
}

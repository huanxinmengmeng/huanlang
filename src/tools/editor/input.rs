
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;
use crate::tools::editor::action::Action;
use crate::tools::editor::cursor::Mode;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Char(char),
    Ctrl(char),
    Alt(char),
    Escape,
    Enter,
    Tab,
    Backspace,
    Delete,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    F(u8),
}

impl Key {
    pub fn from_crossterm(event: KeyEvent) -> Self {
        match event.code {
            KeyCode::Char(c) => {
                if event.modifiers.contains(KeyModifiers::CONTROL) {
                    Key::Ctrl(c)
                } else if event.modifiers.contains(KeyModifiers::ALT) {
                    Key::Alt(c)
                } else {
                    Key::Char(c)
                }
            }
            KeyCode::Esc => Key::Escape,
            KeyCode::Enter => Key::Enter,
            KeyCode::Tab => Key::Tab,
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Delete => Key::Delete,
            KeyCode::Up => Key::Up,
            KeyCode::Down => Key::Down,
            KeyCode::Left => Key::Left,
            KeyCode::Right => Key::Right,
            KeyCode::Home => Key::Home,
            KeyCode::End => Key::End,
            KeyCode::PageUp => Key::PageUp,
            KeyCode::PageDown => Key::PageDown,
            KeyCode::F(n) => Key::F(n),
            _ => Key::Escape,
        }
    }
}

pub struct InputHandler {
    keymap: HashMap<Mode, HashMap<Key, Action>>,
}

impl InputHandler {
    pub fn new() -> Self {
        let mut handler = Self {
            keymap: HashMap::new(),
        };
        handler.init_default_keymaps();
        handler
    }
    
    pub fn handle_key(&self, mode: Mode, key: Key) -> Option<Action> {
        self.keymap
            .get(&mode)
            .and_then(|mode_map| mode_map.get(&key).cloned())
    }
    
    pub fn handle_key_with_fallback(&self, mode: Mode, key: Key) -> Action {
        if let Some(action) = self.handle_key(mode, key.clone()) {
            return action;
        }
        
        match mode {
            Mode::Insert => {
                if let Key::Char(c) = key {
                    return Action::InsertChar(c);
                }
            }
            Mode::Command => {
                if let Key::Char(c) = key {
                    return Action::InsertChar(c);
                }
            }
            _ => {}
        }
        
        Action::None
    }
    
    fn init_default_keymaps(&mut self) {
        let mut normal = HashMap::new();
        normal.insert(Key::Char('h'), Action::MoveLeft);
        normal.insert(Key::Char('j'), Action::MoveDown);
        normal.insert(Key::Char('k'), Action::MoveUp);
        normal.insert(Key::Char('l'), Action::MoveRight);
        normal.insert(Key::Left, Action::MoveLeft);
        normal.insert(Key::Down, Action::MoveDown);
        normal.insert(Key::Up, Action::MoveUp);
        normal.insert(Key::Right, Action::MoveRight);
        
        normal.insert(Key::Char('w'), Action::MoveWordForward);
        normal.insert(Key::Char('b'), Action::MoveWordBackward);
        normal.insert(Key::Char('0'), Action::MoveLineStart);
        normal.insert(Key::Char('$'), Action::MoveLineEnd);
        normal.insert(Key::Home, Action::MoveLineStart);
        normal.insert(Key::End, Action::MoveLineEnd);
        
        normal.insert(Key::Char('g'), Action::MoveDocumentStart);
        normal.insert(Key::Char('G'), Action::MoveDocumentEnd);
        
        normal.insert(Key::Char('i'), Action::EnterMode(Mode::Insert));
        normal.insert(Key::Char('a'), Action::MoveRight);
        normal.insert(Key::Char('o'), Action::EnterMode(Mode::Insert));
        
        normal.insert(Key::Char('v'), Action::EnterMode(Mode::Visual));
        normal.insert(Key::Char('V'), Action::EnterMode(Mode::VisualLine));
        
        normal.insert(Key::Char('x'), Action::DeleteChar);
        normal.insert(Key::Delete, Action::DeleteChar);
        normal.insert(Key::Backspace, Action::DeleteCharBackward);
        normal.insert(Key::Char('d'), Action::DeleteLine);
        normal.insert(Key::Char('D'), Action::DeleteToLineEnd);
        
        normal.insert(Key::Char('y'), Action::Yank);
        normal.insert(Key::Char('Y'), Action::YankLine);
        normal.insert(Key::Char('p'), Action::Paste);
        normal.insert(Key::Char('P'), Action::PasteBefore);
        
        normal.insert(Key::Char('u'), Action::Undo);
        normal.insert(Key::Ctrl('r'), Action::Redo);
        
        normal.insert(Key::Char(':'), Action::EnterMode(Mode::Command));
        
        normal.insert(Key::Char('/'), Action::EnterMode(Mode::Palette));
        
        normal.insert(Key::Escape, Action::EnterMode(Mode::Normal));
        
        normal.insert(Key::Char('s'), Action::Save);
        normal.insert(Key::Ctrl('s'), Action::Save);
        normal.insert(Key::Char('q'), Action::Quit);
        
        self.keymap.insert(Mode::Normal, normal);
        
        let mut insert = HashMap::new();
        insert.insert(Key::Escape, Action::EnterMode(Mode::Normal));
        insert.insert(Key::Ctrl('c'), Action::EnterMode(Mode::Normal));
        insert.insert(Key::Ctrl('h'), Action::DeleteCharBackward);
        insert.insert(Key::Backspace, Action::DeleteCharBackward);
        insert.insert(Key::Delete, Action::DeleteChar);
        insert.insert(Key::Enter, Action::InsertNewline);
        insert.insert(Key::Tab, Action::InsertTab);
        insert.insert(Key::Left, Action::MoveLeft);
        insert.insert(Key::Right, Action::MoveRight);
        insert.insert(Key::Up, Action::MoveUp);
        insert.insert(Key::Down, Action::MoveDown);
        insert.insert(Key::Home, Action::MoveLineStart);
        insert.insert(Key::End, Action::MoveLineEnd);
        
        self.keymap.insert(Mode::Insert, insert);
        
        let mut visual = HashMap::new();
        visual.insert(Key::Escape, Action::EnterMode(Mode::Normal));
        visual.insert(Key::Char('y'), Action::Yank);
        visual.insert(Key::Char('d'), Action::DeleteChar);
        visual.insert(Key::Char('x'), Action::DeleteChar);
        visual.insert(Key::Left, Action::MoveLeft);
        visual.insert(Key::Right, Action::MoveRight);
        visual.insert(Key::Up, Action::MoveUp);
        visual.insert(Key::Down, Action::MoveDown);
        visual.insert(Key::Char('h'), Action::MoveLeft);
        visual.insert(Key::Char('j'), Action::MoveDown);
        visual.insert(Key::Char('k'), Action::MoveUp);
        visual.insert(Key::Char('l'), Action::MoveRight);
        
        self.keymap.insert(Mode::Visual, visual);
        
        let mut visual_line = HashMap::new();
        visual_line.insert(Key::Escape, Action::EnterMode(Mode::Normal));
        visual_line.insert(Key::Char('y'), Action::Yank);
        visual_line.insert(Key::Char('d'), Action::DeleteLine);
        
        self.keymap.insert(Mode::VisualLine, visual_line);
        
        let mut command = HashMap::new();
        command.insert(Key::Escape, Action::EnterMode(Mode::Normal));
        command.insert(Key::Enter, Action::SaveAndQuit);
        command.insert(Key::Backspace, Action::DeleteCharBackward);
        
        self.keymap.insert(Mode::Command, command);
        
        let mut palette = HashMap::new();
        palette.insert(Key::Escape, Action::EnterMode(Mode::Normal));
        palette.insert(Key::Enter, Action::EnterMode(Mode::Normal));
        palette.insert(Key::Backspace, Action::DeleteCharBackward);
        
        self.keymap.insert(Mode::Palette, palette);
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        let mut handler = Self {
            keymap: HashMap::new(),
        };
        // 直接实现 init_default_keymaps 的逻辑
        let mut normal = HashMap::new();
        normal.insert(Key::Char('h'), Action::MoveLeft);
        normal.insert(Key::Char('j'), Action::MoveDown);
        normal.insert(Key::Char('k'), Action::MoveUp);
        normal.insert(Key::Char('l'), Action::MoveRight);
        normal.insert(Key::Left, Action::MoveLeft);
        normal.insert(Key::Down, Action::MoveDown);
        normal.insert(Key::Up, Action::MoveUp);
        normal.insert(Key::Right, Action::MoveRight);
        
        normal.insert(Key::Char('w'), Action::MoveWordForward);
        normal.insert(Key::Char('b'), Action::MoveWordBackward);
        normal.insert(Key::Char('0'), Action::MoveLineStart);
        normal.insert(Key::Char('$'), Action::MoveLineEnd);
        normal.insert(Key::Home, Action::MoveLineStart);
        normal.insert(Key::End, Action::MoveLineEnd);
        
        normal.insert(Key::Char('g'), Action::MoveDocumentStart);
        normal.insert(Key::Char('G'), Action::MoveDocumentEnd);
        
        normal.insert(Key::Char('i'), Action::EnterMode(Mode::Insert));
        normal.insert(Key::Char('a'), Action::MoveRight);
        normal.insert(Key::Char('o'), Action::EnterMode(Mode::Insert));
        
        normal.insert(Key::Char('v'), Action::EnterMode(Mode::Visual));
        normal.insert(Key::Char('V'), Action::EnterMode(Mode::VisualLine));
        
        normal.insert(Key::Char('x'), Action::DeleteChar);
        normal.insert(Key::Delete, Action::DeleteChar);
        normal.insert(Key::Backspace, Action::DeleteCharBackward);
        normal.insert(Key::Char('d'), Action::DeleteLine);
        normal.insert(Key::Char('D'), Action::DeleteToLineEnd);
        
        normal.insert(Key::Char('y'), Action::Yank);
        normal.insert(Key::Char('Y'), Action::YankLine);
        normal.insert(Key::Char('p'), Action::Paste);
        normal.insert(Key::Char('P'), Action::PasteBefore);
        
        normal.insert(Key::Char('u'), Action::Undo);
        normal.insert(Key::Ctrl('r'), Action::Redo);
        
        normal.insert(Key::Char(':'), Action::EnterMode(Mode::Command));
        
        normal.insert(Key::Char('/'), Action::EnterMode(Mode::Palette));
        
        normal.insert(Key::Escape, Action::EnterMode(Mode::Normal));
        
        normal.insert(Key::Char('s'), Action::Save);
        normal.insert(Key::Ctrl('s'), Action::Save);
        normal.insert(Key::Char('q'), Action::Quit);
        
        handler.keymap.insert(Mode::Normal, normal);
        
        let mut insert = HashMap::new();
        insert.insert(Key::Escape, Action::EnterMode(Mode::Normal));
        insert.insert(Key::Ctrl('c'), Action::EnterMode(Mode::Normal));
        insert.insert(Key::Ctrl('h'), Action::DeleteCharBackward);
        insert.insert(Key::Backspace, Action::DeleteCharBackward);
        insert.insert(Key::Delete, Action::DeleteChar);
        insert.insert(Key::Enter, Action::InsertNewline);
        insert.insert(Key::Tab, Action::InsertTab);
        insert.insert(Key::Left, Action::MoveLeft);
        insert.insert(Key::Right, Action::MoveRight);
        insert.insert(Key::Up, Action::MoveUp);
        insert.insert(Key::Down, Action::MoveDown);
        insert.insert(Key::Home, Action::MoveLineStart);
        insert.insert(Key::End, Action::MoveLineEnd);
        
        handler.keymap.insert(Mode::Insert, insert);
        
        let mut visual = HashMap::new();
        visual.insert(Key::Escape, Action::EnterMode(Mode::Normal));
        visual.insert(Key::Char('y'), Action::Yank);
        visual.insert(Key::Char('d'), Action::DeleteChar);
        visual.insert(Key::Char('x'), Action::DeleteChar);
        visual.insert(Key::Left, Action::MoveLeft);
        visual.insert(Key::Right, Action::MoveRight);
        visual.insert(Key::Up, Action::MoveUp);
        visual.insert(Key::Down, Action::MoveDown);
        visual.insert(Key::Char('h'), Action::MoveLeft);
        visual.insert(Key::Char('j'), Action::MoveDown);
        visual.insert(Key::Char('k'), Action::MoveUp);
        visual.insert(Key::Char('l'), Action::MoveRight);
        
        handler.keymap.insert(Mode::Visual, visual);
        
        let mut visual_line = HashMap::new();
        visual_line.insert(Key::Escape, Action::EnterMode(Mode::Normal));
        visual_line.insert(Key::Char('y'), Action::Yank);
        visual_line.insert(Key::Char('d'), Action::DeleteLine);
        
        handler.keymap.insert(Mode::VisualLine, visual_line);
        
        let mut command = HashMap::new();
        command.insert(Key::Escape, Action::EnterMode(Mode::Normal));
        command.insert(Key::Enter, Action::SaveAndQuit);
        command.insert(Key::Backspace, Action::DeleteCharBackward);
        
        handler.keymap.insert(Mode::Command, command);
        
        let mut palette = HashMap::new();
        palette.insert(Key::Escape, Action::EnterMode(Mode::Normal));
        palette.insert(Key::Enter, Action::EnterMode(Mode::Normal));
        palette.insert(Key::Backspace, Action::DeleteCharBackward);
        
        handler.keymap.insert(Mode::Palette, palette);
        
        handler
    }
}

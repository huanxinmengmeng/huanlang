
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::editor::buffer::TextBuffer;
    use crate::tools::editor::cursor::Cursor;
    use crate::tools::editor::history::EditHistory;
    
    #[test]
    fn test_buffer_basic() {
        let mut buffer = TextBuffer::new();
        assert!(buffer.is_empty());
        
        buffer.insert(0, "Hello");
        assert_eq!(buffer.to_string(), "Hello");
        assert_eq!(buffer.len(), 5);
        assert!(!buffer.is_empty());
    }
    
    #[test]
    fn test_buffer_insert_delete() {
        let mut buffer = TextBuffer::from_str("Hello World");
        assert_eq!(buffer.line_count(), 1);
        
        buffer.insert(5, ",");
        assert_eq!(buffer.to_string(), "Hello, World");
        
        buffer.remove(5, 6);
        assert_eq!(buffer.to_string(), "Hello World");
    }
    
    #[test]
    fn test_cursor_position() {
        let buffer = TextBuffer::from_str("Line 1\nLine 2\nLine 3");
        
        let offset = buffer.offset_of_position(1, 0);
        let (line, col) = buffer.position_of_offset(offset);
        assert_eq!(line, 1);
        assert_eq!(col, 0);
    }
    
    #[test]
    fn test_cursor_default() {
        let cursor = Cursor::default();
        assert_eq!(cursor.line, 0);
        assert_eq!(cursor.column, 0);
        assert_eq!(cursor.offset, 0);
        assert!(cursor.anchor.is_none());
    }
}

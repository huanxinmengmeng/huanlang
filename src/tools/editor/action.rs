
use crate::tools::editor::cursor::Mode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    MoveWordForward,
    MoveWordBackward,
    MoveLineStart,
    MoveLineEnd,
    MoveDocumentStart,
    MoveDocumentEnd,
    
    EnterMode(Mode),
    
    InsertChar(char),
    InsertTab,
    InsertNewline,
    DeleteChar,
    DeleteCharBackward,
    DeleteWordForward,
    DeleteWordBackward,
    DeleteLine,
    DeleteToLineEnd,
    
    VisualChar,
    VisualLine,
    VisualBlock,
    
    Yank,
    YankLine,
    Paste,
    PasteBefore,
    
    Undo,
    Redo,
    
    Save,
    SaveAndQuit,
    Quit,
    ForceQuit,
    
    OpenFile,
    FindFile,
    
    GotoDefinition,
    Hover,
    Rename,
    Format,
    
    SwitchTheme,
    
    // 代码折叠
    ToggleFold,
    FoldAll,
    UnfoldAll,
    
    // 多窗口
    SplitHorizontal,
    SplitVertical,
    SwitchWindow,
    CloseWindow,
    
    // 宏录制
    StartMacroRecording,
    StopMacroRecording,
    PlayMacro,
    
    // LSP集成
    ShowCompletions,
    GoToDefinition,
    GoToDeclaration,
    GoToImplementation,
    FindReferences,
    DocumentSymbol,
    WorkspaceSymbol,
    
    None,
}

#[derive(Debug, Clone, Default)]
pub struct CommandResult {
    pub should_quit: bool,
}

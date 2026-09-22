//! Data models for the BabyDra Notepad text editor.

pub mod cursor;
pub mod document;
pub mod editor;
pub mod settings;

pub use cursor::CursorPosition;
pub use document::DocumentInfo;
pub use editor::EditorState;
pub use settings::{load_notepad_cfg, save_notepad_cfg, NotepadSettings};

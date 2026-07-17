//! Drawing and state models for the screenshot editor.

/// Types of drawing annotations that can be overlayed on the screenshot.
#[derive(Clone)]
pub enum Drawing {
    /// Vector path drawing with points, color, and thickness.
    Stroke {
        points: Vec<(f64, f64)>,
        color: (f64, f64, f64),
        width: f64,
    },
    /// A simple outlined rectangle.
    Rect {
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        color: (f64, f64, f64),
        width: f64,
    },
    /// A pixelated area to conceal sensitive information.
    Blur {
        x: f64,
        y: f64,
        w: f64,
        h: f64,
    },
}

/// Tools available in the screenshot editor.
#[derive(Clone, Copy, PartialEq)]
pub enum Tool {
    Select,
    Pen,
    Rect,
    Blur,
    Eraser,
}

/// Current active state of the editor.
pub struct EditorState {
    pub bg_pixbuf: gdk_pixbuf::Pixbuf,
    pub crop_x: f64,
    pub crop_y: f64,
    pub crop_w: f64,
    pub crop_h: f64,
    pub has_selection: bool,
    pub drag_start_x: f64,
    pub drag_start_y: f64,
    pub is_selecting: bool,
    pub current_tool: Tool,
    pub current_color: (f64, f64, f64),
    pub drawings: Vec<Drawing>,
    pub active_stroke: Option<Vec<(f64, f64)>>,
    pub active_rect: Option<(f64, f64, f64, f64)>,
}

impl EditorState {
    /// Creates a new editor state with the provided raw background pixbuf.
    pub fn new(pixbuf: gdk_pixbuf::Pixbuf) -> Self {
        Self {
            bg_pixbuf: pixbuf,
            crop_x: 0.0,
            crop_y: 0.0,
            crop_w: 0.0,
            crop_h: 0.0,
            has_selection: false,
            drag_start_x: 0.0,
            drag_start_y: 0.0,
            is_selecting: false,
            current_tool: Tool::Select,
            current_color: (0.93, 0.15, 0.15),
            drawings: Vec::new(),
            active_stroke: None,
            active_rect: None,
        }
    }
}

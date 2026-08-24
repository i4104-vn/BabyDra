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
    /// An outlined ellipse inscribed in the bounding box.
    Ellipse {
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        color: (f64, f64, f64),
        width: f64,
    },
    /// A straight line segment from (x1, y1) to (x2, y2).
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: (f64, f64, f64),
        width: f64,
    },
    /// A straight arrow from (x1, y1) pointing to (x2, y2).
    Arrow {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: (f64, f64, f64),
        width: f64,
    },
    /// A pixelated area to conceal sensitive information.
    Blur { x: f64, y: f64, w: f64, h: f64 },
}

impl Drawing {
    /// Bounding box of the drawing in canvas coordinates.
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        const STROKE_PAD: f64 = 8.0;
        match self {
            Drawing::Stroke { points, width, .. } => {
                let pad = width.max(STROKE_PAD);
                let mut min = (f64::MAX, f64::MAX);
                let mut max = (f64::MIN, f64::MIN);
                for (x, y) in points {
                    min.0 = min.0.min(*x);
                    min.1 = min.1.min(*y);
                    max.0 = max.0.max(*x);
                    max.1 = max.1.max(*y);
                }
                (
                    min.0 - pad,
                    min.1 - pad,
                    max.0 - min.0 + pad * 2.0,
                    max.1 - min.1 + pad * 2.0,
                )
            }
            Drawing::Rect { x, y, w, h, .. }
            | Drawing::Ellipse { x, y, w, h, .. }
            | Drawing::Blur { x, y, w, h } => (*x, *y, *w, *h),
            Drawing::Line {
                x1,
                y1,
                x2,
                y2,
                width,
                ..
            }
            | Drawing::Arrow {
                x1,
                y1,
                x2,
                y2,
                width,
                ..
            } => {
                let pad = width.max(STROKE_PAD);
                (
                    x1.min(*x2) - pad,
                    y1.min(*y2) - pad,
                    (x1 - x2).abs() + pad * 2.0,
                    (y1 - y2).abs() + pad * 2.0,
                )
            }
        }
    }

    /// Returns `true` if the point is close enough to the drawing to select it.
    pub fn hit_test(&self, px: f64, py: f64) -> bool {
        match self {
            Drawing::Stroke { points, width, .. } => {
                let threshold = (width / 2.0 + 6.0).powi(2);
                points
                    .iter()
                    .any(|(x, y)| (px - x).powi(2) + (py - y).powi(2) <= threshold)
            }
            Drawing::Rect {
                x, y, w, h, width, ..
            } => {
                let t = width / 2.0 + 4.0;
                near_rect_outline(px, py, *x, *y, *w, *h, t)
            }
            Drawing::Ellipse {
                x, y, w, h, width, ..
            } => {
                let rx = w / 2.0;
                let ry = h / 2.0;
                if rx <= 0.0 || ry <= 0.0 {
                    return false;
                }
                let nx = (px - (x + rx)) / rx;
                let ny = (py - (y + ry)) / ry;
                let dist = (nx * nx + ny * ny).sqrt();
                ((dist - 1.0).abs() * rx.min(ry)) <= width / 2.0 + 4.0
            }
            Drawing::Blur { x, y, w, h } => px >= *x && px <= x + w && py >= *y && py <= y + h,
            Drawing::Line {
                x1,
                y1,
                x2,
                y2,
                width,
                ..
            }
            | Drawing::Arrow {
                x1,
                y1,
                x2,
                y2,
                width,
                ..
            } => distance_to_segment(px, py, *x1, *y1, *x2, *y2) <= width / 2.0 + 6.0,
        }
    }
}

/// Squared-distance-free check for being within `t` of a rectangle outline.
fn near_rect_outline(px: f64, py: f64, x: f64, y: f64, w: f64, h: f64, t: f64) -> bool {
    let inside_x = px >= x - t && px <= x + w + t;
    let inside_y = py >= y - t && py <= y + h + t;
    let on_left = (px - x).abs() <= t;
    let on_right = (px - (x + w)).abs() <= t;
    let on_top = (py - y).abs() <= t;
    let on_bottom = (py - (y + h)).abs() <= t;
    (inside_y && (on_left || on_right)) || (inside_x && (on_top || on_bottom))
}

/// Perpendicular distance from point to line segment.
pub fn distance_to_segment(px: f64, py: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let len_sq = dx * dx + dy * dy;
    if len_sq == 0.0 {
        return ((px - x1).powi(2) + (py - y1).powi(2)).sqrt();
    }
    let t = (((px - x1) * dx + (py - y1) * dy) / len_sq).clamp(0.0, 1.0);
    let cx = x1 + t * dx;
    let cy = y1 + t * dy;
    ((px - cx).powi(2) + (py - cy).powi(2)).sqrt()
}

/// Tools available in the screenshot editor.
#[derive(Clone, Copy, PartialEq)]
pub enum Tool {
    Select,
    Pen,
    Rect,
    Ellipse,
    Line,
    Arrow,
    Blur,
    Eraser,
}

/// Default stroke widths offered by the style popover.
pub const STROKE_WIDTHS: [f64; 3] = [3.0, 5.5, 9.0];

/// Current active state of the editor.
pub struct EditorState {
    pub bg_pixbuf: gdk_pixbuf::Pixbuf,
    pub crop_x: f64,
    pub crop_y: f64,
    pub crop_w: f64,
    pub crop_h: f64,
    pub canvas_w: f64,
    pub canvas_h: f64,
    pub has_selection: bool,
    pub drag_start_x: f64,
    pub drag_start_y: f64,
    pub is_selecting: bool,
    pub current_tool: Tool,
    pub current_color: (f64, f64, f64),
    pub current_width: f64,
    pub drawings: Vec<Drawing>,
    pub selected_drawing: Option<usize>,
    pub active_stroke: Option<Vec<(f64, f64)>>,
    pub active_rect: Option<(f64, f64, f64, f64)>,
    pub active_line: Option<(f64, f64, f64, f64)>,
    /// Incremented whenever static content (background/crop/drawings) changes so the
    /// canvas render cache can detect invalidation cheaply.
    pub render_version: u64,
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
            canvas_w: 0.0,
            canvas_h: 0.0,
            has_selection: false,
            drag_start_x: 0.0,
            drag_start_y: 0.0,
            is_selecting: false,
            current_tool: Tool::Select,
            current_color: (0.93, 0.15, 0.15),
            current_width: STROKE_WIDTHS[1],
            drawings: Vec::new(),
            selected_drawing: None,
            active_stroke: None,
            active_rect: None,
            active_line: None,
            render_version: 0,
        }
    }

    /// Marks static content as changed so cached renders are invalidated.
    pub fn invalidate(&mut self) {
        self.render_version += 1;
    }
}

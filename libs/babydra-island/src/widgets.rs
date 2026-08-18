//! Shared widget re-exports.
//!
//! Giữ re-export `notification` để giữ tương thích với `babydra-panel`
//! (`babydra_island::widgets::notification::*`). Các widget riêng của feature
//! đã được chuyển vào bên trong thư mục feature tương ứng.

pub use ::babydra_core::services::notification::service as notification;

pub use babydra_utils::components::{
    clear_list_box, clear_box,
    create_icon_badge,
    PlaceholderState, create_placeholder_row,
};


/// Spawns a background task thread and invokes the `on_done` callback on the main GTK thread upon completion.
pub fn spawn_async_task<T, F, G>(task: F, on_done: G, poll_ms: u64)
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
    G: FnOnce(T) + 'static,
{
    let (tx, rx) = std::sync::mpsc::channel::<T>();
    std::thread::spawn(move || {
        let res = task();
        let _ = tx.send(res);
    });

    let mut on_done_opt = Some(on_done);
    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(poll_ms), move || {
        if let Ok(res) = rx.try_recv() {
            if let Some(cb) = on_done_opt.take() {
                cb(res);
            }
            gtk4::glib::ControlFlow::Break
        } else {
            gtk4::glib::ControlFlow::Continue
        }
    });
}

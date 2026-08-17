use babydra_core::ContentViewHandle;
use gtk4::prelude::*;

/// Update content view ui silent.
pub fn update_content_view_ui_silent(handle: &ContentViewHandle) {
    update_content_view_ui_internal(handle, true);
}

/// Update content view ui.
pub fn update_content_view_ui(handle: &ContentViewHandle) {
    update_content_view_ui_internal(handle, false);
}

/// Update content view ui internal.
fn update_content_view_ui_internal(handle: &ContentViewHandle, silent: bool) {
    let widgets = handle.widgets.clone();
    let entries = handle.entries.borrow().clone();
    let nav_callback = handle.nav_callback.clone();
    let current_path = handle.current_path.borrow().clone();
    let start_path = current_path.clone();
    let current_mode = handle.current_mode.borrow().clone();
    let sort_mode = handle.sort_mode.borrow().clone();
    let selected_paths = handle.selected_paths.clone();
    let handle_c = handle.clone();

    // Increment and capture the render generation
    let gen = {
        let mut g = handle.render_generation.borrow_mut();
        *g += 1;
        *g
    };

    if !silent {
        widgets.progress_bar.set_visible(true);
        widgets.progress_bar.set_fraction(0.0);
    }

    while let Some(child) = widgets.grid_container.first_child() {
        widgets.grid_container.remove(&child);
    }

    while let Some(child) = widgets.listbox.first_child() {
        widgets.listbox.remove(&child);
    }

    glib::spawn_future_local(async move {
        if current_mode == "icons" {
            if sort_mode == "auto" {
                super::grid_renderer::render_flat_grid(
                    &handle_c,
                    &widgets,
                    &entries,
                    &current_path,
                    &start_path,
                    gen,
                    &nav_callback,
                    selected_paths,
                )
                .await;
            } else {
                super::grid_renderer::render_grouped_grid(
                    &handle_c,
                    &widgets,
                    &entries,
                    &current_path,
                    &start_path,
                    gen,
                    &sort_mode,
                    &nav_callback,
                    selected_paths,
                )
                .await;
            }
        } else {
            super::list_renderer::render_list_view(
                &handle_c,
                &widgets,
                &entries,
                &current_path,
                &start_path,
                gen,
                &sort_mode,
                &nav_callback,
                selected_paths,
            )
            .await;
        }

        // Hide progress bar when layout completes successfully
        if *handle_c.current_path.borrow() == start_path
            && *handle_c.render_generation.borrow() == gen
        {
            handle_c.widgets.progress_bar.set_visible(false);
        }
    });
}

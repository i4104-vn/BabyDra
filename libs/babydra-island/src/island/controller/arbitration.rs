//! Controller loop tick and view priority arbitration.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use crate::island::manager::core::IslandCore;
use crate::island::models::IslandDisplay;
use crate::island::ui::transition::apply_transition;
use crate::island::view::IslandCtx;

/// One controller tick: timers → feature ticks → bracket sync → arbitration → transitions.
pub(crate) fn island_tick(core_rc: &Rc<RefCell<IslandCore>>) {
    // Safe try_borrow_mut to prevent re-entrant panics
    let Ok(mut core) = core_rc.try_borrow_mut() else {
        return;
    };
    let now = Instant::now();

    // 1. Auto-hide / auto-release timers.
    for v in core.views.iter() {
        let should_hide = v
            .state
            .auto_hide_at
            .borrow()
            .map(|deadline| now >= deadline)
            .unwrap_or(false);
        if should_hide {
            v.state.requested.set(false);
            v.state.auto_hide_at.borrow_mut().take();
        }

        let should_release = v
            .state
            .release_at
            .borrow()
            .map(|deadline| now >= deadline)
            .unwrap_or(false);
        if should_release {
            v.state.override_active.set(false);
            v.state.release_at.borrow_mut().take();
        }
    }

    // Clean up user_selected if no longer active or superseded by a newer request
    if let Some((sel_idx, sel_seq)) = core.user_selected {
        let mut clear = true;
        if sel_idx < core.views.len() {
            let v = &core.views[sel_idx];
            let is_active = v.state.override_active.get() || v.state.requested.get();
            let newer_request = core.views.iter().enumerate().any(|(i, other)| {
                i != sel_idx
                    && (other.state.override_active.get() || other.state.requested.get())
                    && other.state.request_seq.get() > sel_seq
            });
            if is_active && !newer_request {
                clear = false;
            }
        }
        if clear {
            core.user_selected = None;
        }
    }

    // 2. Feature ticks (every registered feature, every tick).
    let current_idx = match core.displayed {
        IslandDisplay::View(i) => Some(i),
        _ => None,
    };
    let mut ctxs = Vec::new();
    for (i, v) in core.views.iter().enumerate() {
        if let Some(f) = &v.feature {
            ctxs.push((
                f.clone(),
                IslandCtx {
                    capsule: core.capsule.clone(),
                    current: current_idx == Some(i),
                    hovered: core.hovered.get(),
                },
            ));
        }
    }
    for (f, ctx) in ctxs {
        if let Ok(mut feat) = f.try_borrow_mut() {
            feat.tick(&ctx);
        }
    }

    // 3. Update bracket indicators visibility based on active view count
    core.update_brackets();

    // 4. Arbitration.
    let winner = select_winner(&core);
    let desired = match winner {
        Some(w) => IslandDisplay::View(w),
        None if core.cfg.idle_visible && core.idle.is_some() => IslandDisplay::Idle,
        None => IslandDisplay::Hidden,
    };

    // 5. Transitions (cancel previous expand animations immediately if a different view is requested)
    if desired != core.displayed {
        core.animating.set(false);
    } else if core.animating.get() {
        return;
    }

    if let Some(pending) = core.pending.take() {
        if pending != core.displayed {
            apply_transition(&mut core, pending, core_rc);
            return;
        }
    }
    if desired != core.displayed {
        apply_transition(&mut core, desired, core_rc);
    }
}

/// Picks the view to display: active overrides first (most recently requested
/// wins ties), then highest priority (ties broken by most recent request;
/// equal-priority ties also fall back to registration order).
pub(crate) fn select_winner(core: &IslandCore) -> Option<usize> {
    // Honor explicit user scroll selection as long as the view is still active and un-superseded
    if let Some((sel_idx, sel_seq)) = core.user_selected {
        if sel_idx < core.views.len() {
            let v = &core.views[sel_idx];
            let is_active = v.state.override_active.get() || v.state.requested.get();
            if is_active {
                let newer_request = core.views.iter().enumerate().any(|(i, other)| {
                    i != sel_idx
                        && (other.state.override_active.get() || other.state.requested.get())
                        && other.state.request_seq.get() > sel_seq
                });
                if !newer_request {
                    return Some(sel_idx);
                }
            }
        }
    }

    let mut override_best: Option<(u64, usize)> = None;
    for (i, v) in core.views.iter().enumerate() {
        if !v.state.override_active.get() {
            continue;
        }
        let seq = v.state.request_seq.get();
        if override_best.map(|(s, _)| seq > s).unwrap_or(true) {
            override_best = Some((seq, i));
        }
    }
    if let Some((_, i)) = override_best {
        return Some(i);
    }

    let mut best: Option<(u8, u64, usize)> = None;
    for (i, v) in core.views.iter().enumerate() {
        let wanted = v.state.requested.get();
        if !wanted {
            continue;
        }
        let key = (v.priority, v.state.request_seq.get(), i);
        if best.map(|b| key > b).unwrap_or(true) {
            best = Some(key);
        }
    }
    best.map(|(_, _, i)| i)
}

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

    // 1. Auto-hide / auto-release timers and purge dead/expired views from queue.
    core.purge_expired(now);

    // Clean up user_selected if no longer alive or superseded by a newer request
    if let Some((sel_idx, sel_seq)) = core.user_selected.get() {
        let mut clear = true;
        if core.is_view_alive(sel_idx, now) {
            let newer_request = core.views.iter().enumerate().any(|(i, other)| {
                i != sel_idx
                    && core.is_view_alive(i, now)
                    && other.state.request_seq.get() > sel_seq
            });
            if !newer_request {
                clear = false;
            }
        }
        if clear {
            core.user_selected.set(None);
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
    let now = Instant::now();

    // 0. If any view currently has an open popover, it MUST remain displayed!
    // Background events (like media player updates or notifications) must never
    // hijack the capsule while the user is actively viewing or interacting with a popover.
    for (i, v) in core.views.iter().enumerate() {
        if let Some(f) = &v.feature {
            if let Ok(feat) = f.try_borrow() {
                if feat.is_popover_open() {
                    return Some(i);
                }
            }
        }
    }

    // Honor explicit user scroll selection as long as the view is still alive and un-superseded
    if let Some((sel_idx, sel_seq)) = core.user_selected.get() {
        if core.is_view_alive(sel_idx, now) {
            let newer_request = core.views.iter().enumerate().any(|(i, other)| {
                i != sel_idx
                    && core.is_view_alive(i, now)
                    && other.state.request_seq.get() > sel_seq
            });
            if !newer_request {
                return Some(sel_idx);
            }
        }
    }

    let mut override_best: Option<(u64, usize)> = None;
    for (i, v) in core.views.iter().enumerate() {
        if !core.is_view_alive(i, now) || !v.state.override_active.get() {
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
        if !core.is_view_alive(i, now) || !v.state.requested.get() {
            continue;
        }
        let key = (v.priority, v.state.request_seq.get(), i);
        if best.map(|b| key > b).unwrap_or(true) {
            best = Some(key);
        }
    }
    best.map(|(_, _, i)| i)
}

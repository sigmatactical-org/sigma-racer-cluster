//! Window navigation while moving vs stopped.

use sigma_instrumentation::SigmaDashboard;
use slint::ComponentHandle;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::session::{Session, TICK};

pub fn wire(ui: &SigmaDashboard, session: &Rc<RefCell<Session>>) {
    let stopped = Rc::new(Cell::new(true));

    {
        let session = session.clone();
        let stopped = stopped.clone();
        let ui_weak = ui.as_weak();
        ui.on_nav_next(move || {
            if let Some(ui) = ui_weak.upgrade() {
                step(&ui, session.clone(), stopped.get(), 1);
            }
        });
    }
    {
        let session = session.clone();
        let stopped = stopped.clone();
        let ui_weak = ui.as_weak();
        ui.on_nav_prev(move || {
            if let Some(ui) = ui_weak.upgrade() {
                step(&ui, session.clone(), stopped.get(), -1);
            }
        });
    }
    {
        let session = session.clone();
        let ui_weak = ui.as_weak();
        ui.on_nav_home(move || {
            if let Some(ui) = ui_weak.upgrade() {
                session.borrow_mut().current_window = 0;
                ui.set_current_window(0);
            }
        });
    }
    {
        let session = session.clone();
        let stopped = stopped.clone();
        let ui_weak = ui.as_weak();
        ui.on_nav_select(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                select(&ui, session.clone(), stopped.get(), idx);
            }
        });
    }

    let session_tick = session.clone();
    let stopped_tick = stopped.clone();
    let ui_weak = ui.as_weak();
    slint::Timer::default().start(slint::TimerMode::Repeated, TICK, move || {
        if let Some(ui) = ui_weak.upgrade() {
            stopped_tick.set(ui.get_speed() == 0);
            ui.set_nav_blocked_hint(!stopped_tick.get() && session_tick.borrow().current_window >= 4);
        }
    });
}

fn step(ui: &SigmaDashboard, session: Rc<RefCell<Session>>, stopped: bool, delta: i32) {
    let mut session = session.borrow_mut();
    let max = if stopped { 8 } else { 3 };
    let next = (session.current_window + delta).clamp(0, max);
    session.current_window = next;
    ui.set_current_window(next);
}

fn select(ui: &SigmaDashboard, session: Rc<RefCell<Session>>, stopped: bool, idx: i32) {
    let max = if stopped { 8 } else { 3 };
    let next = idx.clamp(0, max);
    session.borrow_mut().current_window = next;
    ui.set_current_window(next);
}

//! Live Connectivity window: BlueZ + connman over D-Bus, driven by face buttons.

mod bluez;
mod connman;

use bluez::BlueZ;
use connman::ConnMan;
use sigma_instrumentation::connectivity::{
    Action, BackResult, Controller, Snapshot, WINDOW as CONN_WINDOW,
};
use sigma_instrumentation::updates::{self as updates_nav, WINDOW as UPDATES_WINDOW};
use sigma_instrumentation::{camera, windows, SigmaDashboard};
use slint::ComponentHandle;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use crate::telemetry::session::{Session, TICK};

struct Backends {
    bluez: Option<BlueZ>,
    connman: Option<ConnMan>,
}

impl Backends {
    fn open() -> Self {
        let bluez = match BlueZ::connect() {
            Ok(b) => Some(b),
            Err(err) => {
                eprintln!("sigma-racer-cluster: BlueZ D-Bus unavailable: {err}");
                None
            }
        };
        let connman = match ConnMan::connect() {
            Ok(c) => Some(c),
            Err(err) => {
                eprintln!("sigma-racer-cluster: connman D-Bus unavailable: {err}");
                None
            }
        };
        Self { bluez, connman }
    }

    fn refresh(&self, snap: &mut Snapshot) {
        snap.available = self.bluez.is_some() || self.connman.is_some();
        let mut notes = Vec::new();

        if let Some(bt) = &self.bluez {
            match bt.powered() {
                Ok(on) => snap.bt_powered = on,
                Err(err) => notes.push(format!("BT: {err}")),
            }
            match bt.devices() {
                Ok((devices, connected)) => {
                    snap.devices = devices;
                    if let Some((name, batt)) = connected {
                        snap.bt_connected = true;
                        snap.bt_device = name;
                        snap.bt_battery = batt;
                    } else {
                        snap.bt_connected = false;
                        if !snap.bt_powered {
                            snap.bt_device.clear();
                        }
                        snap.bt_battery = -1;
                    }
                }
                Err(err) => notes.push(format!("BT devices: {err}")),
            }
        } else {
            notes.push("BlueZ offline".into());
        }

        if let Some(cm) = &self.connman {
            match cm.wifi_powered() {
                Ok(on) => snap.wifi_powered = on,
                Err(err) => notes.push(format!("Wi-Fi: {err}")),
            }
            match cm.networks() {
                Ok((networks, online)) => {
                    snap.networks = networks;
                    if let Some(ssid) = online {
                        snap.wifi_connected = true;
                        snap.wifi_ssid = ssid;
                    } else {
                        snap.wifi_connected = false;
                        if !snap.wifi_powered {
                            snap.wifi_ssid.clear();
                        }
                    }
                }
                Err(err) => notes.push(format!("Wi-Fi nets: {err}")),
            }
        } else {
            notes.push("connman offline".into());
        }

        if !snap.busy {
            if !snap.available {
                snap.status = notes.join(" · ");
            } else if snap.status.contains("offline") || snap.status.starts_with("Starting") {
                snap.status.clear();
            }
        }
    }

    fn run_action(&self, action: Action, snap: &mut Snapshot) {
        snap.busy = true;
        let result = match action {
            Action::ToggleBt => {
                if let Some(bt) = &self.bluez {
                    let on = !snap.bt_powered;
                    bt.set_powered(on)
                        .map(|_| {
                            snap.bt_powered = on;
                            format!("Bluetooth {}", if on { "on" } else { "off" })
                        })
                        .map_err(|e| e.to_string())
                } else {
                    Err("BlueZ unavailable".into())
                }
            }
            Action::OpenBtList | Action::BtScan => {
                if let Some(bt) = &self.bluez {
                    bt.start_discovery()
                        .map(|_| "Scanning for headsets…".into())
                        .map_err(|e| e.to_string())
                } else {
                    Err("BlueZ unavailable".into())
                }
            }
            Action::ToggleWifi => {
                if let Some(cm) = &self.connman {
                    let on = !snap.wifi_powered;
                    cm.set_wifi_powered(on)
                        .map(|_| format!("Wi-Fi {}", if on { "on" } else { "off" }))
                        .map_err(|e| e.to_string())
                } else {
                    Err("connman unavailable".into())
                }
            }
            Action::OpenWifiList | Action::WifiScan => {
                if let Some(cm) = &self.connman {
                    cm.scan_wifi()
                        .map(|_| "Scanning Wi-Fi…".into())
                        .map_err(|e| e.to_string())
                } else {
                    Err("connman unavailable".into())
                }
            }
            Action::SelectDevice(i) => {
                if let Some(bt) = &self.bluez {
                    if let Some(dev) = snap.devices.get(i).cloned() {
                        if dev.connected {
                            bt.disconnect_device(&dev.path)
                                .map(|_| format!("Disconnected {}", dev.title))
                                .map_err(|e| e.to_string())
                        } else {
                            bt.connect_device(&dev.path)
                                .map(|_| format!("Connecting {}", dev.title))
                                .map_err(|e| e.to_string())
                        }
                    } else {
                        Err("No device".into())
                    }
                } else {
                    Err("BlueZ unavailable".into())
                }
            }
            Action::SelectNetwork(i) => {
                if let Some(cm) = &self.connman {
                    if let Some(net) = snap.networks.get(i).cloned() {
                        if net.connected {
                            cm.disconnect_service(&net.path)
                                .map(|_| format!("Disconnected {}", net.title))
                                .map_err(|e| e.to_string())
                        } else if net.badge == "SECURE" && !net.favorite {
                            Err("Password required — provision with connmanctl first".into())
                        } else {
                            cm.connect_service(&net.path)
                                .map(|_| format!("Connecting {}", net.title))
                                .map_err(|e| e.to_string())
                        }
                    } else {
                        Err("No network".into())
                    }
                } else {
                    Err("connman unavailable".into())
                }
            }
        };
        match result {
            Ok(msg) => snap.status = msg,
            Err(err) => snap.status = err,
        }
        snap.busy = false;
        self.refresh(snap);
    }
}

/// Wire Connectivity polling and face-button navigation (incl. in-window focus).
pub fn start(ui: &SigmaDashboard, session: &Rc<RefCell<Session>>) {
    let ctrl = Rc::new(RefCell::new(Controller::new()));
    let backends = Rc::new(Backends::open());

    {
        let mut c = ctrl.borrow_mut();
        backends.refresh(&mut c.snap);
        c.paint(ui);
    }

    wire_nav(ui, session, ctrl.clone(), backends.clone());

    let ui_weak = ui.as_weak();
    let ctrl_tick = ctrl;
    let backends_tick = backends;
    let timer = slint::Timer::default();
    timer.start(slint::TimerMode::Repeated, Duration::from_secs(2), move || {
        let Some(ui) = ui_weak.upgrade() else {
            return;
        };
        let mut c = ctrl_tick.borrow_mut();
        backends_tick.refresh(&mut c.snap);
        if ui.get_current_window() == CONN_WINDOW {
            c.paint(&ui);
        }
    });
    std::mem::forget(timer);
}

fn wire_nav(
    ui: &SigmaDashboard,
    session: &Rc<RefCell<Session>>,
    ctrl: Rc<RefCell<Controller>>,
    backends: Rc<Backends>,
) {
    let stopped = Rc::new(Cell::new(true));

    {
        let session = session.clone();
        let stopped = stopped.clone();
        let ctrl = ctrl.clone();
        let ui_weak = ui.as_weak();
        ui.on_nav_next(move || {
            if let Some(ui) = ui_weak.upgrade() {
                step_or_focus(&ui, &session, &ctrl, stopped.get(), 1);
            }
        });
    }
    {
        let session = session.clone();
        let stopped = stopped.clone();
        let ctrl = ctrl.clone();
        let ui_weak = ui.as_weak();
        ui.on_nav_prev(move || {
            if let Some(ui) = ui_weak.upgrade() {
                step_or_focus(&ui, &session, &ctrl, stopped.get(), -1);
            }
        });
    }
    {
        let session = session.clone();
        let ctrl = ctrl.clone();
        let ui_weak = ui.as_weak();
        ui.on_nav_back(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut session = session.borrow_mut();
            if session.current_window == CONN_WINDOW {
                let mut c = ctrl.borrow_mut();
                match c.menu.back() {
                    BackResult::Stay => {
                        c.paint(&ui);
                        return;
                    }
                    BackResult::LeaveWindow => {
                        c.menu.reset();
                    }
                }
            }
            if session.current_window == UPDATES_WINDOW {
                updates_nav::reset_focus(&ui);
            }
            session.current_window = 0;
            ui.set_current_window(0);
        });
    }
    {
        let session = session.clone();
        let ctrl = ctrl.clone();
        let backends = backends.clone();
        let ui_weak = ui.as_weak();
        ui.on_nav_select(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let win = session.borrow().current_window;
            if win == CONN_WINDOW {
                let mut c = ctrl.borrow_mut();
                let snap = c.snap.clone();
                if let Some(action) = c.menu.select(&snap) {
                    backends.run_action(action, &mut c.snap);
                }
                c.paint(&ui);
            } else if win == UPDATES_WINDOW {
                updates_nav::activate_focused(&ui);
            } else if win == camera::WINDOW {
                camera::toggle_feed(&ui);
            }
        });
    }

    let session_tick = session.clone();
    let stopped_tick = stopped;
    let ui_weak = ui.as_weak();
    slint::Timer::default().start(slint::TimerMode::Repeated, TICK, move || {
        if let Some(ui) = ui_weak.upgrade() {
            stopped_tick.set(ui.get_speed() == 0);
            ui.set_nav_blocked_hint(
                !stopped_tick.get() && session_tick.borrow().current_window > windows::PANEL_MAX,
            );
        }
    });
}

fn max_window(stopped: bool) -> i32 {
    if stopped {
        windows::COUNT - 1
    } else {
        windows::PANEL_MAX
    }
}

fn step_or_focus(
    ui: &SigmaDashboard,
    session: &Rc<RefCell<Session>>,
    ctrl: &Rc<RefCell<Controller>>,
    stopped: bool,
    delta: i32,
) {
    let mut session = session.borrow_mut();
    if session.current_window == CONN_WINDOW {
        let mut c = ctrl.borrow_mut();
        let snap = c.snap.clone();
        if let Some(leave) = c.menu.move_focus(&snap, delta) {
            let next = (CONN_WINDOW + leave).clamp(0, max_window(stopped));
            c.menu.reset();
            session.current_window = next;
            ui.set_current_window(next);
            if next == UPDATES_WINDOW {
                updates_nav::reset_focus(ui);
            }
            return;
        }
        c.paint(ui);
        return;
    }

    if session.current_window == UPDATES_WINDOW {
        if let Some(leave) = updates_nav::move_focus(ui, delta) {
            let next = (UPDATES_WINDOW + leave).clamp(0, max_window(stopped));
            updates_nav::reset_focus(ui);
            session.current_window = next;
            ui.set_current_window(next);
            if next == CONN_WINDOW {
                ctrl.borrow_mut().menu.reset();
                ctrl.borrow_mut().paint(ui);
            }
            return;
        }
        return;
    }

    let next = (session.current_window + delta).clamp(0, max_window(stopped));
    if next == CONN_WINDOW {
        ctrl.borrow_mut().menu.reset();
    }
    if next == UPDATES_WINDOW {
        updates_nav::reset_focus(ui);
    }
    session.current_window = next;
    ui.set_current_window(next);
    if next == CONN_WINDOW {
        ctrl.borrow_mut().paint(ui);
    }
}

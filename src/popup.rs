// src/popup.rs
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label, CssProvider};
use gtk4_layer_shell::{Layer, LayerShell};
use glib::ControlFlow;
use std::sync::OnceLock;
use std::sync::mpsc::{self, Sender, Receiver};
use std::cell::RefCell;

static TX: OnceLock<Sender<(String, i32, i32)>> = OnceLock::new();

fn get_mouse_position() -> (i32, i32) {
    if let Ok(output) = std::process::Command::new("hyprctl")
        .args(["cursorpos"])
        .output()
    {
        let s = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = s.trim().split(',').collect();
        if parts.len() == 2 {
            let x = parts[0].trim().parse().unwrap_or(0);
            let y = parts[1].trim().parse().unwrap_or(0);
            return (x, y);
        }
    }
    if let Ok(output) = std::process::Command::new("xdotool")
        .args(["getmouselocation", "--shell"])
        .output()
    {
        let s = String::from_utf8_lossy(&output.stdout);
        let mut x = 0i32;
        let mut y = 0i32;
        for line in s.lines() {
            if let Some(val) = line.strip_prefix("X=") {
                x = val.parse().unwrap_or(0);
            } else if let Some(val) = line.strip_prefix("Y=") {
                y = val.parse().unwrap_or(0);
            }
        }
        return (x, y);
    }
    (0, 0)
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(
        "
        window {
            background: transparent;
        }

        .popup-box {
            background-color: rgba(18, 18, 22, 0.92);
            border-radius: 12px;
            border: 1px solid rgba(255, 255, 255, 0.08);
            box-shadow: 0 4px 24px rgba(0, 0, 0, 0.5);
            padding: 10px 16px;
        }

        .popup-label {
            color: rgba(230, 230, 240, 0.95);
            font-family: 'Inter', 'Noto Sans', sans-serif;
            font-size: 13px;
            font-weight: 400;
            letter-spacing: 0.2px;
        }
        ",
    );

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub struct Popup;

impl Popup {
    pub fn init_gtk_app() {
        let (tx, rx) = mpsc::channel::<(String, i32, i32)>();
        TX.set(tx).expect("init_gtk_app chamado mais de uma vez");

        let app = Application::builder()
            .application_id("com.waytranslate.popup")
            .build();

        let rx_slot: RefCell<Option<Receiver<(String, i32, i32)>>> = RefCell::new(Some(rx));

        app.connect_activate(move |app| {
            load_css();

            let label = Label::builder()
                .label("")
                .wrap(true)
                .max_width_chars(48)
                .build();
            label.add_css_class("popup-label");

            let container = gtk4::Box::builder()
                .orientation(gtk4::Orientation::Vertical)
                .build();
            container.add_css_class("popup-box");
            container.append(&label);

            let window = ApplicationWindow::builder()
                .application(app)
                .child(&container)
                .decorated(false)
                .build();

            // Janela transparente para bordas arredondadas funcionarem
            window.set_opacity(1.0);

            window.init_layer_shell();
            window.set_layer(Layer::Overlay);
            window.set_anchor(gtk4_layer_shell::Edge::Top, true);
            window.set_anchor(gtk4_layer_shell::Edge::Left, true);

            let (mx, my) = get_mouse_position();
            window.set_margin(gtk4_layer_shell::Edge::Top, my + 16);
            window.set_margin(gtk4_layer_shell::Edge::Left, mx + 12);

            window.present();

            if let Some(rx) = rx_slot.borrow_mut().take() {
                let label_clone = label.clone();
                let window_clone = window.clone();

                glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                    while let Ok((text, x, y)) = rx.try_recv() {
                        label_clone.set_text(&text);
                        window_clone.set_margin(gtk4_layer_shell::Edge::Top, y + 16);
                        window_clone.set_margin(gtk4_layer_shell::Edge::Left, x + 12);
                    }
                    ControlFlow::Continue
                });
            }
        });

        app.run_with_args::<&str>(&[]);
    }

    pub fn show(text: String) -> anyhow::Result<()> {
        if let Some(tx) = TX.get() {
            let (x, y) = get_mouse_position();
            tx.send((text, x, y))?;
        }
        Ok(())
    }
}

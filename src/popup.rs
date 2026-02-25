// src/popup.rs
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label};
use gtk4_layer_shell::{Layer, LayerShell};
use glib::ControlFlow;
use std::sync::OnceLock;
use std::sync::mpsc::{self, Sender, Receiver};
use std::cell::RefCell;

static TX: OnceLock<Sender<(String, i32, i32)>> = OnceLock::new();

/// Obtém a posição atual do cursor via `xdotool` (X11) ou `wlr-randr`/`hyprctl` (Wayland)
fn get_mouse_position() -> (i32, i32) {
    // Tenta via hyprctl (Hyprland)
    if let Ok(output) = std::process::Command::new("hyprctl")
        .args(["cursorpos"])
        .output()
    {
        let s = String::from_utf8_lossy(&output.stdout);
        // formato: "123, 456"
        let parts: Vec<&str> = s.trim().split(',').collect();
        if parts.len() == 2 {
            let x = parts[0].trim().parse().unwrap_or(0);
            let y = parts[1].trim().parse().unwrap_or(0);
            return (x, y);
        }
    }

    // Fallback: xdotool (X11)
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
            let label = Label::builder()
                .label("Aguardando tradução...")
                .margin_top(15)
                .margin_bottom(15)
                .margin_start(15)
                .margin_end(15)
                .build();

            let window = ApplicationWindow::builder()
                .application(app)
                .child(&label)
                .build();

            window.init_layer_shell();
            window.set_layer(Layer::Overlay);

            // Ancora nas edges Top e Left para poder posicionar via margem
            window.set_anchor(gtk4_layer_shell::Edge::Top, true);
            window.set_anchor(gtk4_layer_shell::Edge::Left, true);

            // Posição inicial no cursor
            let (mx, my) = get_mouse_position();
            window.set_margin(gtk4_layer_shell::Edge::Top, my);
            window.set_margin(gtk4_layer_shell::Edge::Left, mx);

            window.present();

            if let Some(rx) = rx_slot.borrow_mut().take() {
                let label_clone = label.clone();
                let window_clone = window.clone();

                glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                    while let Ok((text, x, y)) = rx.try_recv() {
                        label_clone.set_text(&text);

                        // Reposiciona a janela para onde o mouse estava no momento do show()
                        window_clone.set_margin(gtk4_layer_shell::Edge::Top, y);
                        window_clone.set_margin(gtk4_layer_shell::Edge::Left, x);
                    }
                    ControlFlow::Continue
                });
            }
        });

        app.run_with_args::<&str>(&[]);
    }

    pub fn show(text: String) -> anyhow::Result<()> {
        if let Some(tx) = TX.get() {
            // Captura a posição do mouse no momento em que show() é chamado
            let (x, y) = get_mouse_position();
            tx.send((text, x, y))?;
        }
        Ok(())
    }
}

// src/main.rs
mod app;
mod clipboard;
mod translator;
mod popup;
mod hotkey;

use popup::Popup;

fn main() {
    tracing_subscriber::fmt::init();

    // Tokio roda numa thread separada
    std::thread::spawn(|| {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                // Aguarda o GTK inicializar
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                let app = app::App::new().await.unwrap();
                loop {
                    if let Err(e) = app.run().await {
                        eprintln!("Erro: {e}");
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                }
            });
    });

    // GTK DEVE rodar na main thread
    Popup::init_gtk_app();
}

mod app;
mod audio;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Zay Audio Player",
        native_options,
        Box::new(|_cc| Ok(Box::new(app::AudioPlayerApp::new()))),
    )
}

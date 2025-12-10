use crate::audio::AudioEngine;
use eframe::egui::{Context, Ui};
use std::path::PathBuf;

pub struct AudioPlayerApp {
    engine: AudioEngine,
    current_file: Option<PathBuf>,
    volume: f32,
}

impl AudioPlayerApp {
    pub fn new() -> Self {
        let engine = AudioEngine::new().expect("audio init failed");
        Self {
            engine,
            current_file: None,
            volume: 0.5,
        }
    }
    fn ui_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Play").clicked() {
                self.engine.play();
            }
            if ui.button("Pause").clicked() {
                self.engine.pause();
            }

            ui.label("Volume");
            let mut vol = self.volume;
            if ui.add(egui::Slider::new(&mut vol, 0.0..=1.0)).changed() {
                self.volume = vol;
                self.engine.set_volume(self.volume);
            }
        });
    }
}

impl eframe::App for AudioPlayerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Zay's Audio Player (demo)");

            if ui.button("Load example file").clicked() {
                // In reality you’d use a file dialog; hard-coded path here:
                let path = PathBuf::from("File name here");
                if let Err(err) = self.engine.load_and_play_file(&path) {
                    eprintln!("Failed to play file: {err}");
                } else {
                    self.current_file = Some(path);
                }
            }

            self.ui_controls(ui);
        });
    }
}

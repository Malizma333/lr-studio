use eframe::egui;
use lr_physics_engine::Engine;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Line Rider Studio",
        native_options,
        Box::new(|cc| Ok(Box::new(LineRiderStudioApp::new(cc)))),
    );
}

#[derive(Default)]
struct LineRiderStudioApp {
    physics_engine: Engine,
}

impl LineRiderStudioApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for LineRiderStudioApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Line Rider Studio",
        native_options,
        Box::new(|cc| Ok(Box::new(LineRiderStudioApp::new(cc)))),
    );
}

#[derive(Default)]
struct LineRiderStudioApp {}

impl LineRiderStudioApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for LineRiderStudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| ui.label("Hello world!"));
    }
}

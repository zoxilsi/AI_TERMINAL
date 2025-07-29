use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Terminal App",
        options,
        Box::new(|_cc| Box::new(TerminalApp::default())),
    )
}

#[derive(Default)]
struct TerminalApp {
    input: String,
}

impl eframe::App for TerminalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Terminal");
            ui.text_edit_singleline(&mut self.input);
        });
    }
}

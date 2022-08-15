use eframe::egui;

pub struct SpotifyAnalyzerApp;

impl SpotifyAnalyzerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for SpotifyAnalyzerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("My egui Application");
                ui.label("Test");
            });

            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Click each year").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

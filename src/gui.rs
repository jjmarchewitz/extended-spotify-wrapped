use eframe::{
    egui::{self, style},
    epaint::{Color32, Rounding, Stroke},
};

pub struct ExtendedSpotifyWrappedApp;

impl ExtendedSpotifyWrappedApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure custom fonts
        setup_custom_fonts(&cc.egui_ctx);

        // Set custom UI colors
        cc.egui_ctx.set_visuals({
            let visuals = style::Visuals {
                dark_mode: true,
                widgets: style::Widgets {
                    noninteractive: style::WidgetVisuals {
                        bg_fill: Color32::from_gray(18), // window background
                        bg_stroke: Stroke::new(1.0, Color32::from_gray(60)), // separators, indentation lines, windows outlines
                        fg_stroke: Stroke::new(1.0, Color32::from_gray(140)), // normal text color
                        rounding: Rounding::same(2.0),
                        expansion: 0.0,
                    },
                    ..Default::default()
                },
                ..Default::default()
            };

            visuals
        });

        Self {}
    }
}

impl eframe::App for ExtendedSpotifyWrappedApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top collapsable filter menu
        egui::TopBottomPanel::top("top_filter_panel").show(ctx, |ui| {
            egui::CollapsingHeader::new("Search")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label("text");
                });
        });

        // Search results display
        egui::CentralPanel::default().show(ctx, |ui| {

            // egui::Frame {
            //     rounding: Rounding::none(),
            //     ..Default::default()
            // }
            // .show(ui, |ui| {
            //     ui.horizontal(|ui| {
            //         ui.add_space(50.);
            //         ui.vertical(|ui| {
            //             ui.label("text");
            //         });
            //         ui.vertical_centered(|ui| {
            //             ui.heading("My egui Application");
            //             ui.label("Test");
            //         });
            //     });
            // });
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "CircularStd-Medium".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/CircularStd-Medium.otf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "CircularStd-Medium".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("CircularStd-Medium".to_owned());

    ctx.set_fonts(fonts);
}

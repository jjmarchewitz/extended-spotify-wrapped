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
                        bg_stroke: Stroke::new(1.0, Color32::from_rgb(43, 41, 54)), // separators, indentation lines, windows outlines
                        fg_stroke: Stroke::new(1.0, Color32::from_gray(255)), // normal text color
                        rounding: Rounding::same(2.0),
                        expansion: 0.0,
                    },
                    inactive: style::WidgetVisuals {
                        bg_fill: Color32::from_gray(60), // button background
                        bg_stroke: Default::default(),
                        fg_stroke: Stroke::new(1.0, Color32::from_gray(180)), // button text
                        rounding: Rounding::same(2.0),
                        expansion: 0.0,
                    },
                    hovered: style::WidgetVisuals {
                        bg_fill: Color32::from_gray(70),
                        bg_stroke: Stroke::new(1.0, Color32::from_gray(150)), // e.g. hover over window edge or button
                        fg_stroke: Stroke::new(1.5, Color32::from_gray(240)),
                        rounding: Rounding::same(3.0),
                        expansion: 1.0,
                    },
                    active: style::WidgetVisuals {
                        bg_fill: Color32::from_gray(55),
                        bg_stroke: Stroke::new(1.0, Color32::WHITE),
                        fg_stroke: Stroke::new(2.0, Color32::WHITE),
                        rounding: Rounding::same(2.0),
                        expansion: 1.0,
                    },
                    open: style::WidgetVisuals {
                        bg_fill: Color32::from_gray(27),
                        bg_stroke: Stroke::new(1.0, Color32::from_gray(60)),
                        fg_stroke: Stroke::new(1.0, Color32::from_gray(210)),
                        rounding: Rounding::same(2.0),
                        expansion: 0.0,
                    },
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
        render_top_filter_bar(ctx);

        // Search results display
        render_filter_output(ctx);
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

fn render_top_filter_bar(ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_filter_panel").show(ctx, |ui| {
        egui::CollapsingHeader::new("Filter")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("text");
            });

        // let id = ui.make_persistent_id("Filter");
        // egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false)
        //     .show_header(ui, |ui| {
        //         ui.label("Filter"); // you can put checkboxes or whatever here
        //     })
        //     .body(|ui| {
        //         ui.label("BODD");
        //     });
    });
}

fn render_filter_output(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::Frame {
            rounding: Rounding::none(),
            ..Default::default()
        }
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("My egui Application");
                    ui.label("Test");
                });
            });
        });
    });
}

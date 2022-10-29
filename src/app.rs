use crate::json_loading;
use eframe::epaint::{Color32, Rounding, Stroke};
use egui::{style, FontData, FontDefinitions, FontFamily};
use std::path;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ESWApp {
    // Example stuff:
    app_title: String,

    // File loading
    raw_data_path: Option<path::PathBuf>,
    #[serde(skip)]
    data_is_loaded: bool,
    #[serde(skip)]
    loaded_data: Option<Vec<json_loading::PlayedItem>>,
    #[serde(skip)]
    attempted_to_load_data: bool,
    #[serde(skip)]
    loading_error_msg: Option<String>,
}

impl Default for ESWApp {
    fn default() -> Self {
        Self {
            app_title: "Extended Spotify Wrapped".to_owned(),
            data_is_loaded: false,
            raw_data_path: None,
            loaded_data: None,
            attempted_to_load_data: false,
            loading_error_msg: None,
        }
    }
}

impl eframe::App for ESWApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.data_is_loaded {
            self.loading_screen(ctx, _frame);
        } else {
            self.data_screen(ctx, _frame);
        }
    }
}

impl ESWApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // FIXME: Custom fonts don't work when compiled for wasm
        #[cfg(not(target_arch = "wasm32"))]
        ESWApp::set_custom_fonts(&cc);

        // Set up custom visuals
        ESWApp::set_custom_visuals(&cc);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    /// Sets the font to be Circular Std.
    fn set_custom_fonts(cc: &eframe::CreationContext<'_>) {
        // Mutably get the default fonts
        let mut fonts = FontDefinitions::default();

        // Define the font name
        let custom_font_name = "CircularStd".to_owned();

        // Add the custom font to the default fonts
        fonts.font_data.insert(
            custom_font_name.clone(),
            FontData::from_static(include_bytes!("../assets/fonts/CircularStd-Medium.otf")),
        );

        // Put custom font as the first priority for proportional fonts
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, custom_font_name.clone());

        // Put custom font as last fallback for monospace:
        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .push(custom_font_name.clone());

        // Tell egui to use the new fonts
        cc.egui_ctx.set_fonts(fonts);
    }

    /// Sets up custom visual settings
    fn set_custom_visuals(cc: &eframe::CreationContext<'_>) {
        let visuals = egui::Visuals {
            dark_mode: true,
            widgets: style::Widgets {
                noninteractive: style::WidgetVisuals {
                    bg_fill: Color32::from_gray(18), // window background
                    bg_stroke: Stroke::new(1.0, Color32::from_rgb(43, 41, 54)), // separators, indentation lines, windows outlines
                    fg_stroke: Stroke::new(1.0, Color32::from_gray(255)),       // normal text color
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
                ..Default::default()
            },
            ..Default::default()
        };

        // Tell egui to use the new visuals
        cc.egui_ctx.set_visuals(visuals);
    }

    /// Display the loading screen to let the user select the folder containing their spotify data
    fn loading_screen(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(ctx.available_rect().height() / 3.);
                ui.label("Select the unzipped folder containing your Spotify data.");

                // If the open folder button is pressed, open a file dialog and store the result
                if ui.button("Open Folder").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.raw_data_path = Some(path);
                    }
                }

                // If a data path has been selected, attempt to load it
                if let Some(path) = &self.raw_data_path {
                    // TODO: Load data on separate thread?
                    match json_loading::extract_song_plays_from_json_files_at_path(&path) {
                        Ok(data) => {
                            self.loaded_data = Some(data);
                            self.data_is_loaded = true;
                        }
                        Err(e) => {
                            // Clear self.raw_data_path if loading the persisted path does not work
                            if !self.attempted_to_load_data {
                                self.raw_data_path = None;
                            }
                            // Else show the given error message
                            else {
                                self.loading_error_msg = Some(e.to_string())
                            }
                        }
                    }

                    self.attempted_to_load_data = true;
                }

                // Display an error message if there is one
                if let Some(error_str) = &self.loading_error_msg {
                    ui.label(error_str);
                }
            })
        });
    }

    /// Display the data screen to let the user analyze their data
    /// TODO: Add button to choose new data folder
    fn data_screen(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.add_space(6.);
            ui.heading("Filter");
            ui.separator();
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.heading("Bottom Panel");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Results");
            ui.separator();
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Center).with_cross_justify(true),
                |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for i in 1..63 {
                            ui.label(format!("text {}", u64::pow(2, i)).to_owned());
                        }
                    });
                },
            );
        });
    }
}

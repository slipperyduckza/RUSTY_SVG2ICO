use eframe::egui;
use std::io;

// Embed the logo image data at compile time so it's included in the executable
static LOGO_DATA: &[u8] = include_bytes!("../RUSTYSVG2ICO420.png");

// Main application struct that holds the state of our SVG to ICO converter app
struct SvgToIcoApp {
    // Raw ICO file data in memory
    ico_data: Option<Vec<u8>>,
    // List of textures for each icon size, with their resolution strings
    textures: Vec<(egui::TextureHandle, String)>,
    // True if the ICO was generated from SVG, false if loaded from file
    is_generated: bool,
    // Texture for the app logo
    logo_texture: Option<egui::TextureHandle>,
}

// Implement Default trait to create a new app instance with initial state
impl Default for SvgToIcoApp {
    fn default() -> Self {
        Self {
            ico_data: None,
            textures: vec![],
            is_generated: false,
            logo_texture: None,
        }
    }
}

impl SvgToIcoApp {
    // Load ICO data and create textures for each icon size for display
    fn load_textures(&mut self, ctx: &egui::Context, ico_data: &[u8]) {
        let icon_dir = ico::IconDir::read(io::Cursor::new(ico_data)).unwrap();
        self.textures.clear();
        for entry in icon_dir.entries() {
            let img = if entry.is_png() {
                image::load_from_memory_with_format(&entry.data(), image::ImageFormat::Png).unwrap()
            } else {
                image::load_from_memory_with_format(&entry.data(), image::ImageFormat::Bmp).unwrap()
            };
            let rgba = img.to_rgba8();
            let size = [entry.width() as usize, entry.height() as usize];
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &rgba);
            let texture = ctx.load_texture(
                format!("ico_{}x{}", entry.width(), entry.height()),
                color_image,
                egui::TextureOptions::default(),
            );
            self.textures.push((texture, format!("{} x {}", entry.width(), entry.height())));
        }
    }
}

// Implement the App trait for eframe to define our UI and logic
impl eframe::App for SvgToIcoApp {
    // This function is called every frame to update the UI
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Define the stroke color for buttons
        let dark_slate_grey = egui::Color32::from_rgb(47, 79, 79);
        // Load the logo texture if not already loaded
        if self.logo_texture.is_none() {
            let img = image::load_from_memory(LOGO_DATA).unwrap();
            let rgba = img.to_rgba8();
            let size = [img.width() as usize, img.height() as usize];
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &rgba);
            self.logo_texture = Some(ctx.load_texture("logo", color_image, egui::TextureOptions::default()));
        }
        // Create the main UI panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // Display the logo centered at the top
            ui.vertical_centered(|ui| {
                ui.image((self.logo_texture.as_ref().unwrap().id(), self.logo_texture.as_ref().unwrap().size_vec2() * 0.5));
            });

            // Horizontal layout for the main action buttons
            ui.horizontal(|ui| {
                // Button to select an SVG file and convert it to ICO
                if ui.add_sized([140.0, 35.0], egui::Button::new("Select SVG File").stroke(egui::Stroke::new(3.0, dark_slate_grey))).clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("SVG", &["svg"]).pick_file() {
                        // Create a temporary directory and file for the conversion
                        let temp_dir = tempfile::TempDir::new().unwrap();
                        let temp_path = temp_dir.path().join("temp.ico");
                        // Convert SVG to ICO with multiple sizes
                        svg_to_ico::svg_to_ico(&path, 256.0, &temp_path, &[256u16, 128, 64, 48, 32, 24, 16]).unwrap();
                        // Read the generated ICO data
                        let ico_data = std::fs::read(&temp_path).unwrap();
                        self.ico_data = Some(ico_data.clone());
                        self.is_generated = true;
                        // Load textures for display
                        self.load_textures(ctx, &ico_data);
                    }
                }

                // Right-align the "Open ICO File" button
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Button to load an existing ICO file for viewing
                    if ui.add_sized([140.0, 35.0], egui::Button::new("Open ICO File").stroke(egui::Stroke::new(3.0, dark_slate_grey))).clicked() {
                        if let Some(path) = rfd::FileDialog::new().add_filter("ICO", &["ico"]).pick_file() {
                            // Read the ICO file data
                            let ico_data = std::fs::read(&path).unwrap();
                            self.ico_data = Some(ico_data.clone());
                            self.is_generated = false;
                            // Load textures for display
                            self.load_textures(ctx, &ico_data);
                        }
                    }
                });
            });

            // Add some space above the save button
            ui.add_space(4.0);

            // Show save button only if we have generated ICO data
            if self.ico_data.is_some() && self.is_generated {
                // Button to save the generated ICO to a file
                if ui.add_sized([140.0, 35.0], egui::Button::new("Save Icon").stroke(egui::Stroke::new(3.0, dark_slate_grey))).clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("ICO", &["ico"]).save_file() {
                        // Write the ICO data to the chosen file
                        std::fs::write(path, self.ico_data.as_ref().unwrap()).unwrap();
                    }
                }
            }

            // Add space before the display box
            ui.add_space(6.0);

            // Center the display box vertically
            ui.vertical_centered(|ui| {
                // Create a framed group for the icon display area
                egui::Frame::group(&ui.style()).fill(egui::Color32::DARK_GRAY).rounding(10.0).inner_margin(6.0).show(ui, |ui| {
                    // Allocate space for the scrollable area
                    ui.allocate_ui(egui::vec2(380.0, 596.0), |ui| {
                        // Make it scrollable vertically
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            // If no textures, show empty space
                            if self.textures.is_empty() {
                                ui.allocate_space(egui::vec2(368.0, 400.0));
                            } else {
                                // Display each icon size with its resolution
                                for (texture, res) in &self.textures {
                                let image_size = texture.size_vec2();
                                let row_height = image_size.y;
                                ui.horizontal(|ui| {
                                    ui.allocate_ui(egui::vec2(262.0, row_height), |ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.image((texture.id(), image_size));
                                        });
                                    });
                                    ui.allocate_ui(egui::vec2(368.0 - 262.0, row_height), |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| ui.colored_label(egui::Color32::WHITE, res));
                                    });
                                    });
                                });
                            }
                        }
                    });
                });
            });
            });
        });
    }
}

// Main function to start the application
fn main() -> eframe::Result<()> {
    // Set up window options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([420.0, 800.0]),
        ..Default::default()
    };
    // Run the native app with our SvgToIcoApp
    eframe::run_native(
        "Rusty SVG2ICO",
        options,
        Box::new(|_cc| Box::new(SvgToIcoApp::default())),
    )
}
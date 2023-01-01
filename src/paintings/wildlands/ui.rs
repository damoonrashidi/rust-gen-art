use std::{fmt::Debug, fs::File, io::Write};

use super::{algo::wildlands, config::WildlandsConfig};

pub struct WildlandsUi {
    svg: egui_extras::RetainedImage,
    config: WildlandsConfig,
    svg_str: String,
}

impl Debug for WildlandsUi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Default for WildlandsUi {
    fn default() -> Self {
        WildlandsUi {
            svg: egui_extras::RetainedImage::from_svg_str(
                "default",
                r#"<svg viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg"></svg>"#,
            )
            .unwrap(),
            config: WildlandsConfig {
                seed: 0,
                line_count: 400,
                size: 800.0,
                chaos: 0.5,
                smoothness: 400.0,
                max_line_length: 200,
            },
            svg_str: "".into(),
        }
    }
}

impl WildlandsUi {
    fn generate(&self) -> String {
        let svg = wildlands(&self.config);
        svg.generate()
    }

    pub fn set_new_svg(&mut self) {
        let svg_str = self.generate();
        self.svg_str = svg_str.clone();
        self.svg = egui_extras::RetainedImage::from_svg_str("Wildlands", svg_str.as_str()).unwrap();
    }
}

impl eframe::App for WildlandsUi {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            if ui
                .add(eframe::egui::Slider::new(&mut self.config.seed, 0..=10_000).text("Seed"))
                .changed()
            {
                self.set_new_svg();
            }

            if ui
                .add(eframe::egui::Slider::new(&mut self.config.size, 800.0..=3000.0).text("Size"))
                .changed()
            {
                self.set_new_svg();
            }

            if ui
                .add(
                    eframe::egui::Slider::new(&mut self.config.line_count, 800..=3_000)
                        .text("Line count"),
                )
                .changed()
            {
                self.set_new_svg();
            }

            if ui
                .add(
                    eframe::egui::Slider::new(&mut self.config.chaos, 0.2..=10.0)
                        .text("Chaos")
                        .step_by(0.2),
                )
                .changed()
            {
                self.set_new_svg();
            }

            if ui
                .add(
                    eframe::egui::Slider::new(&mut self.config.smoothness, 100.0..=2000.0)
                        .text("Smoothness")
                        .step_by(100.0),
                )
                .changed()
            {
                self.set_new_svg();
            }

            if ui
                .add(
                    eframe::egui::Slider::new(
                        &mut self.config.max_line_length,
                        100..=self.config.size as usize,
                    )
                    .text("Max line length"),
                )
                .changed()
            {
                self.set_new_svg();
            }

            if ui.button("Save").clicked() {
                let mut f = File::create("./output/forces/forces-live.svg")
                    .expect("could not open file for writing");

                f.write_all(self.svg_str.as_bytes())
                    .expect("Could not write to file");
            }

            self.svg.show_size(ui, ui.available_size());
        });
    }
}

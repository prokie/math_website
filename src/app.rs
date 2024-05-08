/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// if we add new fields, give them default values when deserializing old state
use eframe::egui;
use egui::*;
use egui_plot::{Legend, Line, Plot, PlotPoints, Points};
use std::f64::consts::TAU;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    points: Vec<[f64; 2]>,
    darts_to_throw: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            points: vec![],
            darts_to_throw: "0".to_string(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Approximating the Value of Pi");

            let my_plot = Plot::new("My Plot").legend(Legend::default());

            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            ui.label(format!(
                "The number of darths thrown: {}",
                self.points.len()
            ));
            ui.label(format!(
                "The number of darths inside the circle: {}",
                self.points
                    .iter()
                    .filter(|point| (point[0] * point[0]) + (point[1] * point[1]) <= 1.0)
                    .count()
            ));

            ui.horizontal(|ui| {
                let name_label = ui.label("How many darts to throw?");
                ui.text_edit_singleline(&mut self.darts_to_throw)
                    .labelled_by(name_label.id);
            });

            if ui.button("Throw darts").clicked() {
                for _ in 0..self.darts_to_throw.parse::<i32>().unwrap() {
                    let x = rand::random::<f64>() * 2.0 - 1.0;
                    let y = rand::random::<f64>() * 2.0 - 1.0;
                    self.points.push([x, y]);
                }
            }

            let n = 512;

            let circle_points: PlotPoints = (0..=n)
                .map(|i| {
                    let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
                    [t.cos(), t.sin()]
                })
                .collect();

            let circle = Line::new(circle_points).name("circle");

            let rectangle = Line::new(vec![
                [-1.0, -1.0],
                [1.0, -1.0],
                [1.0, 1.0],
                [-1.0, 1.0],
                [-1.0, -1.0],
            ]);

            my_plot.show(ui, |plot_ui| {
                plot_ui.line(circle);
                plot_ui.line(rectangle);
                plot_ui.points(Points::new(self.points.clone()).radius(1.0))
            });
        });
    }
}

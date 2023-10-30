#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

mod requestsender;
mod userentry;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Send RequestV2",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    // name: String,
    // age: u32,
    title: String,
    my_f32: f32,
    my_boolean: bool,
    opt: MyOption,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            title: "test window".to_string(),
            my_f32: 10.0,
            my_boolean: false,
            opt: MyOption::First,
        }
    }
}

#[derive(PartialEq, Debug)]
enum MyOption {
    First,
    Second,
    Third,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Send Request");
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Click each year").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.label("This is a label");
            ui.hyperlink("https://github.com/emilk/egui");
            ui.text_edit_singleline(&mut self.title);

            if ui.button("Click me").clicked() {
                println!("clicked me");
            }
            ui.add(egui::Slider::new(&mut self.my_f32, 0.0..=100.0));
            ui.add(egui::DragValue::new(&mut self.my_f32));

            ui.checkbox(&mut self.my_boolean, "Checkbox");

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.opt, MyOption::First, "First");
                ui.radio_value(&mut self.opt, MyOption::Second, "Second");
                ui.radio_value(&mut self.opt, MyOption::Third, "Third");
            });

            ui.separator();
            ui.collapsing("Click to see what is hidden!", |ui| {
                ui.label("Not much, as it turns out");
            });

            egui::Grid::new("some_unique_id").show(ui, |ui| {
                ui.label("First row, first column");
                ui.label("First row, second column");
                ui.end_row();

                ui.label("Second row, first column");
                ui.label("Second row, second column");
                ui.label("Second row, third column");
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label("Same");
                    ui.label("cell");
                });
                ui.label("Third row, second column");
                ui.end_row();
            });
        });
    }
}

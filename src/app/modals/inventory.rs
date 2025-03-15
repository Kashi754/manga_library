use egui::Ui;

use crate::App;

#[derive(Default)]
pub struct Inventory {}

pub fn show_inventory(ui: &mut Ui, app: &mut App) {
    ui.heading("eframe template");

    ui.horizontal(|ui| {
        ui.label("Write something: ");
        ui.text_edit_singleline(&mut app.label);
    });

    ui.add(egui::Slider::new(&mut app.value, 0.0..=10.0).text("value"));
    if ui.button("Increment").clicked() {
        app.value += 1.0;
    }

    ui.separator();

    ui.add(egui::github_link_file!(
        "https://github.com/emilk/eframe_template/blob/main/",
        "Source code."
    ));
}

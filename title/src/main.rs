use eframe::egui;
use std::process::Command;

pub fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut ip = "127.0.0.1".to_owned();

    let _ = eframe::run_simple_native("se-phere.io", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome To SPHERE.io");
            ui.label(" ");
            ui.horizontal(|ui| {
                let name_label = ui.label("Server IP: ");
                ui.text_edit_singleline(&mut ip).labelled_by(name_label.id);
            });
            ui.label(" ");
            if ui.button("Join Game").clicked() {
                let result = ip.clone();
                let _ = Command::new("target/release/client").args([result]).spawn();
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    });
}

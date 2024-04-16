use eframe::egui;
use std::fs::File;
use std::io::{BufRead, Write};

pub fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut ip = "192.168.0.0".to_owned();
    let mut result = None;

    let _ = eframe::run_simple_native("se-phere.io", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome To SPHERE.io");
            ui.label(" ");
            ui.horizontal(|ui| {
                let name_label = ui.label("Server IP: ");
                ui.text_edit_singleline(&mut ip)
                    .labelled_by(name_label.id);
            });
            ui.label(" ");
            if ui.button("Join Game").clicked() {
                result = Some(ip.clone());
                
                // Write the IP address to a file named "ip.txt"
                if let Some(ip) = &result {
                    if let Err(err) = write_ip_to_file(ip) {
                        eprintln!("Error writing IP to file: {}", err);
                    }
                }
                
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    });
}

fn write_ip_to_file(ip: &str) -> std::io::Result<()> {
    let mut file = File::create("ip.txt")?;
    writeln!(file, "{}", ip)?;
    Ok(())
}

pub fn read_ip_from_file(filename: &str) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut reader = std::io::BufReader::new(file);
    let mut ip = String::new();
    reader.read_line(&mut ip)?;
    Ok(ip.trim().to_string())
}
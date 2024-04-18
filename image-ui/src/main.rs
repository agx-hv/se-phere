use eframe::egui;

pub fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    let mut args = vec![];
    for arg in std::env::args() {
        args.push(arg);
    }

    let title = if args[1] == "lose" {"Game Over :("} else {"You Win :)" };
    eframe::run_native(
        title,
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let mut args = vec![];
                for arg in std::env::args() {
                    args.push(arg);
                }

                let a = args[1].as_str();

                match a {
                    "lose" => ui.image(egui::include_image!("../../assets/screens/gameover.jpg")),
                    "win" => ui.image(egui::include_image!("../../assets/screens/win.png")),
                    _ => todo!(),
                }
            });
        });
    }
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    io::Cursor,
    time::{Duration, Instant},
};

use eframe::egui;
use rodio::{source::Source, Decoder, OutputStream, OutputStreamHandle};

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    //let file = BufReader::new(File::open("src/beep-07a.mp3").unwrap());

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Sourdough Timer",
        options,
        Box::new(|_cc| Box::new(MyApp::new(stream_handle))),
    )
}

struct Task {
    name: String,
    /// in seconds
    end_time: u64,
    silenced: bool,
}

struct MyApp {
    start_time: Instant,
    /// edge detector for seconds
    sec: u64,
    stream_handle: OutputStreamHandle,
    //beepit: bool,
    settings_is_open: bool,
    //autolyse: Task,
    tasks: Vec<Task>,
}

impl MyApp {
    fn new(stream_handle: OutputStreamHandle) -> Self {
        let mut tasks = vec![];
        let mut time = 0;
        for (name, own_time) in [("Autolyse", 60), ("S&F No.1", 30), ("Bulk", 120)] {
            time += 60 * own_time;
            tasks.push(Task {
                name: name.to_owned(),
                end_time: time,
                silenced: false,
            })
        }
        Self {
            start_time: Instant::now(),
            sec: 0,
            stream_handle,
            // beepit: false,
            settings_is_open: false,
            //autolyse: (60, false),
            tasks,
        }
    }
    fn beep(&self) {
        // Create a cursor to the in-memory file using a path relative to this file (.rs)
        let file = Cursor::new(include_bytes!("beep-07a.mp3"));
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        // Play the sound directly on the device
        let _ = self.stream_handle.play_raw(source.convert_samples());
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(16));
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sourdough Timer");

            let s = self.start_time.elapsed().as_secs();
            let beepit = s >= self.autolyse.0 && !self.autolyse.1;

            ui.horizontal(|ui| {
                ui.label("Your name: ");
                //let s = (self.start_time.elapsed().as_millis() / 200) as u64;

                if beepit && s != self.sec {
                    self.beep();
                    self.sec = s;
                }

                //if self.beepit && s != self.sec {
                // Play the sound directly on the device
                //let _ = stream_handle.play_raw(source.convert_samples());
                //    self.beep();
                //    self.sec = s;
                //}

                let h = 0;
                let m = 0;
                ui.text_edit_singleline(&mut format!("{}:{}:{}", h, m, s));
            });

            ui.add(egui::ProgressBar::new(0.0));
            if ui
                .add_enabled(beepit, egui::Button::new("Silence Alarm"))
                .clicked()
            {
                self.start_time = Instant::now();
                self.sec = 0;
                self.autolyse.1 = true;
            }

            if ui.button("Settings").clicked() {
                //self.beep();
                self.settings_is_open = !self.settings_is_open;
            }

            let window = egui::Window::new("Settings")
                //.collapsible(false)
                .resizable(false)
                .default_size((250.0, 200.0))
                .open(&mut self.settings_is_open)
                .anchor(egui::Align2::LEFT_TOP, [0.0, 0.0]);

            window.show(ctx, |ui| {
                // do stuff!
            });
        });
    }
}

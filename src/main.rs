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
    start_time: Option<Instant>,
    /// edge detector for seconds
    sec: u64,
    stream_handle: OutputStreamHandle,
    beepit: bool,
    settings_is_open: bool,
    curr_step: usize,
    tasks: Vec<Task>,
}

impl MyApp {
    fn new(stream_handle: OutputStreamHandle) -> Self {
        let mut tasks = vec![];
        let mut time = 0;
        for (name, own_time) in [
            ("Autolyse", 30),
            ("S&F No.1", 30),
            ("Bulk", 120),
            ("Shaping", 30),
            ("Proofing", 120),
        ] {
            time += 1 * own_time; // time is cumulative of all task's times
            tasks.push(Task {
                name: name.to_owned(),
                end_time: time,
                silenced: false,
            })
        }
        Self {
            start_time: None,
            sec: 0,
            curr_step: 0,
            stream_handle,
            beepit: false,
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

            // let running_time = self.start_time.elapsed().as_secs();
            let mut running_time = 0;
            if let Some(start_time) = self.start_time {
                running_time = start_time.elapsed().as_secs();

                for i in 0..self.tasks.len() {
                    if running_time < self.tasks[i].end_time {
                        if self.curr_step != i {
                            self.beepit = true;
                        }
                        self.curr_step = i;
                        break;
                    }
                }
            }

            // let running_time = self.start_time.elapsed().as_millis() as u64 / 10;
            // let beepit = running_time >= self.tasks[self.curr_step].end_time
            // && !self.tasks[self.curr_step].silenced;

            ui.horizontal(|ui| {
                ui.label("Elapsed Time: ");
                //let s = (self.start_time.elapsed().as_millis() / 200) as u64;

                if self.beepit && running_time != self.sec {
                    self.beep();
                    self.sec = running_time;
                }

                //if self.beepit && s != self.sec {
                // Play the sound directly on the device
                //let _ = stream_handle.play_raw(source.convert_samples());
                //    self.beep();
                //    self.sec = s;
                //}
                let s = running_time % 60;
                let m = (running_time / 60) % 60;
                let h = (running_time / 60) / 60;
                ui.label(format!("{:02}:{:02}:{:02}", h, m, s)); // zero padded string
            });
            ui.label(format!("Current Step: {}", self.tasks[self.curr_step].name));

            ui.add(egui::ProgressBar::new(0.0));
            if ui
                .add_enabled(self.beepit, egui::Button::new("Silence Alarm"))
                .clicked()
            {
                //self.start_time = Instant::now();
                //self.sec = 0;
                //self.tasks[0].silenced = true;
                self.beepit = false;
            }

            if ui.button("Settings").clicked() {
                //self.beep();
                self.settings_is_open = !self.settings_is_open;
            }

            if ui.button("Start").clicked() {
                self.start_time = Some(Instant::now());
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

use crate::task::{load_tasks, save_tasks, Task};
use eframe::egui;

static FILE_PATH: &str = "data/tasks.json";

pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Task Tracker",
        options,
        Box::new(|_cc| Box::new(TaskApp::default())),
    )
}

#[derive(Default)]
struct TaskApp {
    tasks: Vec<Task>,
}

impl eframe::App for TaskApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Task Tracker");

            if ui.button("Load Tasks").clicked() {
                self.tasks = load_tasks(FILE_PATH);
            }

            ui.separator();

            let mut name = String::new();
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut name);
                if ui.button("Add Task").clicked() {
                    let new_task = Task::new(name);
                    self.tasks.push(new_task);
                    save_tasks(&self.tasks, FILE_PATH).ok();
                }
            });
        });
    }
}

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

struct TaskApp {
    tasks: Vec<Task>,
    new_task_name: String,
    needs_refresh: bool,
}

impl Default for TaskApp {
    fn default() -> Self {
        Self {
            tasks: load_tasks(FILE_PATH),
            new_task_name: String::new(),
            needs_refresh: false,
        }
    }
}

impl eframe::App for TaskApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.needs_refresh {
            self.tasks = load_tasks(FILE_PATH);
            self.needs_refresh = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Task Tracker");

            let mut task_to_complete = None;
            let mut task_to_delete = None;

            for (index, task) in self.tasks.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{}{}",
                        task.name,
                        if task.completed { " - (Completed)" } else { "" }
                    ));
                    if !task.completed && ui.button("Complete").clicked() {
                        task_to_complete = Some(index);
                    }
                    if ui.button("Delete").clicked() {
                        task_to_delete = Some(index);
                    }
                });
            }

            if let Some(index) = task_to_complete {
                self.tasks[index].complete();
                save_tasks(&self.tasks, FILE_PATH).ok();
                self.needs_refresh = true;
            }

            if let Some(index) = task_to_delete {
                self.tasks.remove(index);
                save_tasks(&self.tasks, FILE_PATH).ok();
                self.needs_refresh = true;
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_task_name);
                if ui.button("Add Task").clicked() && !self.new_task_name.trim().is_empty() {
                    let new_task = Task::new(self.new_task_name.trim().to_string());
                    self.tasks.push(new_task);
                    save_tasks(&self.tasks, FILE_PATH).ok();
                    self.new_task_name.clear();
                }
            });

            ui.separator();

            if ui.button("Clear Tasks").clicked() {
                self.tasks.clear();
                save_tasks(&self.tasks, FILE_PATH).ok();
                self.needs_refresh = true;
            }
        });
    }
}

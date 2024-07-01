use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Schedule Manager",
        native_options,
        Box::new(|cc| Box::new(ScheduleManager::new(cc))),
    ).unwrap();
}

#[derive(Default)]
struct ScheduleManager {
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
}

impl ScheduleManager {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            start_date: chrono::offset::Utc::now().date_naive(),
            end_date: chrono::offset::Utc::now().date_naive(),
        }
    }
}

impl eframe::App for ScheduleManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.add(egui_extras::DatePickerButton::new(&mut self.start_date));
            ui.end_row();
        });
    }
}

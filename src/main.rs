use eframe::egui;
use chrono::{DateTime, Utc};


fn main() -> Result<(), eframe::Error> {
    println!("Hello, world!");
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<CalendarTimer>::default()),
    )
}

struct Event {
    title: String,
    start: String,
    end: String,
    description: String,
}

struct CalendarTimer {
    caldavUrl: String,
    events: Vec<Event>,
}

impl Default for CalendarTimer {
    fn default() -> Self {
        Self {
            caldavUrl: "https://caldav.fastmail.com/dav/calendars/".to_owned(),
            events: get_mock_events(),
        }
    }
}

impl eframe::App for CalendarTimer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Calendar Timer");
            ui.horizontal(|ui| {
                let name_label = ui.label("CalDAV URL: ");
                ui.text_edit_singleline(&mut self.caldavUrl)
                    .labelled_by(name_label.id);
            });
            ui.heading("Events");
            ui.vertical(|ui| {
                for event in &self.events {
                    ui.horizontal(|ui| {
                        ui.label(&event.title);
                        ui.label(&event.start);
                        ui.label(&event.end);
                        ui.label(&event.description);
                    });
                }
            });
        });
    }
}

fn get_mock_events() -> Vec<Event> {
    let now: DateTime<Utc> = Utc::now();
    let start = now.format("%Y-%m-%dT%H:00:00Z").to_string();

    vec![
        Event {
            title: "Meeting with John".to_owned(),
            // today at 13:00 UTC
            start: now.format("%Y-%m-%dT%H:00:00Z").to_string(),
            // end 1 hour later
            end: now.add(Duration::hours(1)).format("%Y-%m-%dT%H:00:00Z").to_string(),
            description: "Discuss project status".to_owned(),
        },
        Event {
            title: "Lunch with Jane".to_owned(),
            start: "2022-01-02T12:00:00Z".to_owned(),
            end: "2022-01-02T13:00:00Z".to_owned(),
            description: "Catch up on recent news".to_owned(),
        },
        Event {
            title: "Conference call with team".to_owned(),
            start: "2022-01-03T14:00:00Z".to_owned(),
            end: "2022-01-03T15:00:00Z".to_owned(),
            description: "Discuss upcoming features".to_owned(),
        },
    ]
}
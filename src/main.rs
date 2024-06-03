use iced::{executor, Application, Command, Settings, Theme};

fn main() -> iced::Result {
    ScheduleManager::run(Settings::default())
}

struct ScheduleManager;

#[derive(Debug)]
enum Message {}

impl Application for ScheduleManager {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        String::from("schedule manager")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        "Hello World!".into()
    }
}

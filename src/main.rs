use iced::{font, Application, Command, Settings, Theme, widget::{Button, Text, Row, Container}, Alignment, Length};
use iced_aw::{date_picker::Date, helpers::date_picker};

fn main() -> iced::Result {
    ScheduleManager::run(Settings::default())
}

struct ScheduleManager {
    date: Date,
    show_date_picker: bool,
}

#[derive(Clone, Debug)]
enum Message {
    DatePickerFontLoaded(Result<(), font::Error>),
    ChooseDate,
    CancelDate,
    SubmitDate(Date),
}

impl Application for ScheduleManager {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let schedule_manager = Self {date: Date::today(), show_date_picker: false};
        (
            schedule_manager,
            font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::DatePickerFontLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("schedule manager")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChooseDate => self.show_date_picker = true,
            Message::SubmitDate(date) => {
                self.date = date;
                self.show_date_picker = false;
            },
            Message::CancelDate => self.show_date_picker = false,
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let but = Button::new(Text::new("Set Date")).on_press(Message::ChooseDate);

        let datepicker = date_picker(
            self.show_date_picker,
            self.date,
            but,
            Message::CancelDate,
            Message::SubmitDate,
        );

        let row = Row::new()
            .align_items(Alignment::Center)
            .spacing(10)
            .push(datepicker)
            .push(Text::new(format!("Date: {}", self.date)));

        Container::new(row)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

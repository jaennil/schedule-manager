use iced::{
    font,
    widget::{button, text},
    Application, Command, Settings, Theme,
};
use iced_aw::{date_picker::Date, DatePicker};

fn main() -> iced::Result {
    ScheduleManager::run(Settings::default())
}

struct DatePickerState {
    date: Date,
    show: bool,
}

struct ScheduleManager {
    date_picker_state: DatePickerState,
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
        (
            Self {
                date_picker_state: DatePickerState {
                    date: Date::today(),
                    show: false,
                },
            },
            font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::DatePickerFontLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("schedule manager")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChooseDate => self.date_picker_state.show = true,
            Message::SubmitDate(date) => {
                self.date_picker_state.date = date;
                self.date_picker_state.show = false;
            }
            Message::CancelDate => self.date_picker_state.show = false,
            Message::DatePickerFontLoaded(_) => {}
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let date_picker_date = self.date_picker_state.date;

        DatePicker::new(
            self.date_picker_state.show,
            date_picker_date,
            button(text(date_picker_date.to_string())).on_press(Message::ChooseDate),
            Message::CancelDate,
            Message::SubmitDate,
        )
        .into()
    }
}

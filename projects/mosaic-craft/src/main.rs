use iced::{Application, Command, Settings, Theme};
use iced::widget::{column, Column};
use iced::widget::*;

struct MosaicCraft {
    // The counter value
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl MosaicCraft {
    pub fn view(&self) -> Column<Message> {
        let image = Image::new("resources/ferris.png");

        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `IncrementPressed` message when pressed
            button("+").on_press(Message::IncrementPressed),
            image,

            // We show the value of the counter here
            text(self.value).size(50),

            // The decrement button. We tell it to produce a
            // `DecrementPressed` message when pressed
            button("-").on_press(Message::DecrementPressed),
        ]
    }
}

impl MosaicCraft {
    // ...
    pub fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
}

impl Application for MosaicCraft {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (MosaicCraft, Command<Message>) {
        (
            MosaicCraft { value: 0 },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.update(message);

        Command::none()
    }

    fn view(&self) -> iced::Element<Message> {
        self.view().into()
    }
}


fn main() {
    MosaicCraft::run(Settings::default()).unwrap()
}
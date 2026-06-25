use iced::{Fill, Element, padding};
use iced::widget::{Column, Text, button, column, container, text};

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}

//fn it_counts_properly() {
    //use magic::{display, interact};
    //let mut counter = Counter::default();
    //display(&interface);
    //let messages = interact(&interface);
    //for message in messages {
    //counter.update(message);
    //}
//}

impl Counter {
    fn view(&self) -> Element<'_,Message> {
        let CounterWidget = column![
                button("+").on_press(Message::Increment),
                text(self.value),
                button("-").on_press(Message::Decrement)
        ]
        .spacing(10);


        container(
            column![
                text("Counting time"),
                button("+").on_press(Message::Increment),
                text(self.value),
                button("-").on_press(Message::Decrement),
            ]
            .spacing(10)
        )
        .padding(10)
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }
}

//fn view(_: &()) -> Text<'_>{
    //use iced::widget::text;
    //use iced::{Fill, Font};

    //text("Hello World\n")
        //.font(Font::MONOSPACE)
        //.size(30)
        //.line_height(1.5)
        //.width(Fill)
        //.height(Fill)
        //.center()
//}

pub fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
    //iced::run((), view)
}

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

use iced::widget::{Column, button, column, text};

impl Counter {
    fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
        ]
    }
}

pub fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}

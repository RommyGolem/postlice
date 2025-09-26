use iced::{self, Element, Subscription, keyboard};

use postlice::add_screen;

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .subscription(subscription)
        .run()
}
struct State {
    screen: Screen,
}

#[derive(Clone)]
enum Screen {
    Add(add_screen::State),
}

impl Default for State {
    fn default() -> Self {
        Self {
            screen: Screen::Add(add_screen::State::default()),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    AddScreen(add_screen::Message),
}

impl State {
    fn update(&mut self, message: Message) {
        match (&mut self.screen, message) {
            (Screen::Add(state), Message::AddScreen(message)) => state.update(message),
        };
    }

    fn view(&self) -> Element<'_, Message> {
        match &self.screen {
            Screen::Add(state) => state.view().map(Message::AddScreen),
        }
    }
}

fn subscription(_state: &State) -> Subscription<Message> {
    keyboard::on_key_press(|key, _modifier| match key {
        _ => None,
    })
}

#[cfg(test)]
mod test {}

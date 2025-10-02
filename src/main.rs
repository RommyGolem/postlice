use std::borrow::Cow;

use iced::{self, Element, Font, Pixels, Settings, Subscription, keyboard};

use postlice::add_screen;

fn main() -> iced::Result {
    let settings = Settings {
        id: None,
        fonts: vec![Cow::Borrowed(include_bytes!("../data/Sarabun-Regular.ttf"))],
        default_font: Font::with_name("Sarabun"),
        default_text_size: Pixels(20.0),
        antialiasing: false,
    };

    iced::application(State::default, State::update, State::view)
        .settings(settings)
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

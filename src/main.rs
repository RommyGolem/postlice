use iced::{self, Element, Font, Pixels, Settings, Subscription, Task, keyboard};
use postlice::{State, add_screen, subscription};
use std::borrow::Cow;

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

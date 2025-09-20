use iced::{
    self, Element, Fill, Subscription, keyboard,
    widget::{button, column, container, text},
};

fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .subscription(subscription)
        .run()
}

#[derive(Default)]
struct State {
    count: i64,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Increment => state.count += 1,
        Message::Decrement => state.count -= 1,
    }
}

fn view(state: &State) -> Element<'_, Message> {
    container(column![
        button("+").on_press(Message::Increment),
        text(state.count),
        button("-").on_press(Message::Decrement),
    ])
    .center(Fill)
    .into()
}

fn subscription(_state: &State) -> Subscription<Message> {
    keyboard::on_key_press(|key, _modifier| match key {
        keyboard::Key::Named(keyboard::key::Named::ArrowUp) => Some(Message::Increment),
        keyboard::Key::Named(keyboard::key::Named::ArrowDown) => Some(Message::Decrement),
        _ => None,
    })
}

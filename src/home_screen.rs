use iced::{
    Element, Fill, Task,
    widget::{button, container},
};

#[derive(Clone, PartialEq, Default)]
pub struct State;

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    GotoAddScreen,
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GotoAddScreen => Task::done(Message::GotoAddScreen),
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        container(button("เพิ่มรายการ").on_press(Message::GotoAddScreen))
            .center(Fill)
            .into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn goto_add_screen() {
        let mut state = State;
        let task = state.update(Message::GotoAddScreen);
        let _ = task.map(|message| assert_eq!(message, Message::GotoAddScreen));
    }
}

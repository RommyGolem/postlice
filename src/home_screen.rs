use iced::{
    Element, Fill, Task,
    widget::{button, container},
};

#[derive(Clone, PartialEq)]
pub struct State;

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    GotoAddScreen,
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<crate::Message> {
        match message {
            Message::GotoAddScreen => Task::done(crate::Message::GotoAddScreen),
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        container(button("เพิ่มรายการ").on_press(Message::GotoAddScreen))
            .center(Fill)
            .into()
    }
}

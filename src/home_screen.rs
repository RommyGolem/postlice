use iced::{
    Element, Fill, Task,
    widget::{button, container, text},
};

#[derive(Clone)]
pub(crate) struct State;

#[derive(Clone, Debug)]
pub(crate) enum Message {
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

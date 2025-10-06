pub mod add_screen;
pub mod database;
pub mod geo_data;
pub mod home_screen;

use iced::{Element, Subscription, Task, keyboard};

pub struct State {
    screen: Screen,
}

#[derive(Clone)]
enum Screen {
    Add(Box<add_screen::State>),
    Home(home_screen::State),
}

impl Default for State {
    fn default() -> Self {
        Self {
            screen: Screen::Home(home_screen::State),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    HomeScreen(home_screen::Message),
    AddScreen(add_screen::Message),
    GotoAddScreen,
    GotoHome,
    None,
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let mut tasks: Vec<Task<Message>> = Vec::new();

        match (&mut self.screen, message) {
            (_, Message::GotoAddScreen) => {
                self.screen = Screen::Add(Box::from(add_screen::State::default()))
            }
            (_, Message::GotoHome) => self.screen = Screen::Home(home_screen::State),
            (Screen::Home(state), Message::HomeScreen(message)) => {
                tasks.push(state.update(message).map(
                    |message: home_screen::Message| match message {
                        home_screen::Message::GotoAddScreen => Message::GotoAddScreen,
                        // _ => Message::HomeScreen(message),
                    },
                ))
            }
            (Screen::Add(state), Message::AddScreen(message)) => {
                tasks.push(state.update(message).map(
                    |message: add_screen::Message| match message {
                        add_screen::Message::GotoHome => Message::GotoHome,
                        _ => Message::AddScreen(message),
                    },
                ))
            }
            (_, _) => {}
        };
        Task::batch(tasks)
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.screen {
            Screen::Home(state) => state.view().map(Message::HomeScreen),
            Screen::Add(state) => state.view().map(Message::AddScreen),
        }
    }
}

pub fn subscription(_state: &State) -> Subscription<Message> {
    keyboard::on_key_press(|_key, _modifier| None)
}

#[cfg(test)]
mod test {}

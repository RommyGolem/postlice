pub mod add_screen;
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
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let mut tasks = Vec::new();

        match (&mut self.screen, message) {
            (_, Message::GotoAddScreen) => {
                self.screen = Screen::Add(Box::from(add_screen::State::default()))
            }
            (_, Message::GotoHome) => self.screen = Screen::Home(home_screen::State),
            (Screen::Home(state), Message::HomeScreen(message)) => {
                tasks.push(state.update(message))
            }
            (Screen::Add(state), Message::AddScreen(message)) => tasks.push(state.update(message)),
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

pub mod database {
    use sqlx::SqlitePool;
    use std::sync::OnceLock;

    static POOL: OnceLock<SqlitePool> = OnceLock::new();

    pub async fn init_pool() -> sqlx::Result<()> {
        let pool = SqlitePool::connect("sqlite::memory:").await?;
        POOL.set(pool).map_err(|_| sqlx::Error::PoolClosed)?;
        Ok(())
    }

    pub fn get_pool() -> Option<&'static SqlitePool> {
        POOL.get()
    }
}

pub mod add_screen;
pub mod home_screen;

use iced::{Element, Subscription, Task, keyboard};

pub struct State {
    screen: Screen,
}

#[derive(Clone)]
enum Screen {
    Add(add_screen::State),
    Home(home_screen::State),
}

impl Default for State {
    fn default() -> Self {
        Self {
            screen: Screen::Home(home_screen::State),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    HomeScreen(home_screen::Message),
    AddScreen(add_screen::Message),
    GotoAddScreen,
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let mut tasks = Vec::new();

        match (&mut self.screen, message) {
            (_, Message::GotoAddScreen) => self.screen = Screen::Add(add_screen::State::default()),
            (Screen::Home(state), Message::HomeScreen(message)) => {
                tasks.push(state.update(message))
            }
            (Screen::Add(state), Message::AddScreen(message)) => state.update(message),
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

pub mod geo_data {
    use serde::Deserialize;
    use std::sync::LazyLock;

    #[derive(Deserialize, Clone, Default)]
    pub struct Province {
        pub name: String,
        pub districts: Vec<District>,
    }

    #[derive(Deserialize, Clone, Default, PartialEq, Debug)]
    pub struct District {
        pub name: String,
        pub postal_code: u32,
        pub sub_districts: Vec<String>,
    }

    pub static GEO_DATA: LazyLock<Vec<Province>> =
        LazyLock::new(|| serde_json::from_str(include_str!("../data/geo-data.json")).unwrap());
}

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

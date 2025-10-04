pub mod add_screen;

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
    use sqlx::sqlite::SqlitePool;
    use std::sync::OnceLock;

    pub static POOL: OnceLock<SqlitePool> =
        OnceLock::get_or_init(|| async { SqlitePool::connect("sqlite::memory").await.unwrap() });
}

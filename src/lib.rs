use serde::Deserialize;
use std::sync::{Arc, LazyLock};

#[derive(Deserialize, Clone)]
pub struct Province {
    pub name: String,
    pub districts: Vec<District>,
}

#[derive(Deserialize, Clone)]
pub struct District {
    pub name: String,
    pub postal_code: u32,
    pub sub_districts: Vec<String>,
}

pub static GEO_DATA: LazyLock<Vec<Province>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../data/geo-data.json")).unwrap_or_default()
});

use iced::{
    Element,
    Length::Fill,
    widget::{column, combo_box, container, row, text, text_input},
};

use crate::geo_data::GEO_DATA;

#[derive(Clone)]
pub struct State {
    address: Address,
    provinces: combo_box::State<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            address: Address::default(),
            provinces: combo_box::State::with_selection(
                (*GEO_DATA
                    .iter()
                    .map(|province| province.name.clone())
                    .collect::<Vec<String>>())
                .to_vec(),
                None,
            ),
        }
    }
}

#[derive(Default, Clone)]
pub struct Address {
    name: String,
    recipient: String,
    address: String,
    _sub_district: Option<String>,
    _district: Option<String>,
    province: Option<String>,
    _postal_code: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    OnProvinceSelect(String),
    OnProvinceInput(String),
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::OnProvinceSelect(province) => self.address.province = Some(province),
            Message::OnProvinceInput(filter) => {
                self.provinces = combo_box::State::with_selection(
                    (*GEO_DATA
                        .iter()
                        .map(|province| province.name.clone())
                        .filter(|province| province.contains(&filter))
                        .collect::<Vec<String>>())
                    .to_vec(),
                    Some(&filter),
                )
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(column![
            row![text("ชื่อ"), text_input("", self.address.name.as_str())],
            row![text("ถึง"), text_input("", self.address.recipient.as_str())],
            row![text("ที่อยู่"), text_input("", self.address.address.as_str())],
            row![
                text("จังหวัด"),
                combo_box(
                    &self.provinces,
                    "เลือกจังหวัด",
                    self.address.province.as_ref(),
                    Message::OnProvinceSelect
                )
                .on_input(Message::OnProvinceInput)
            ],
        ])
        .center(Fill)
        .into()
    }
}

use iced::{
    Element,
    Length::Fill,
    widget::{column, combo_box, container, row, text, text_input},
};

use crate::geo_data::GEO_DATA;

enum ID {
    Name,
    Recipient,
}

impl ID {
    fn as_str(&self) -> &'static str {
        match self {
            ID::Name => "name_text_input",
            ID::Recipient => "recipient_text_input",
        }
    }
}

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
    OnNameInput(String),
    OnRecipientInput(String),
    OnProvinceSelect(String),
    OnProvinceInput(String),
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::OnNameInput(name) => self.address.name = name,
            Message::OnRecipientInput(recipient) => self.address.recipient = recipient,
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
            row![
                text("ชื่อ"),
                text_input("", self.address.name.as_str())
                    .id(ID::Name.as_str())
                    .on_input(Message::OnNameInput)
            ],
            row![
                text("ถึง"),
                text_input("", self.address.recipient.as_str())
                    .id(ID::Recipient.as_str())
                    .on_input(Message::OnRecipientInput)
            ],
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

#[cfg(test)]
mod test {

    use super::{ID, State};
    use iced_test::{selector, simulator};

    #[test]
    fn name_input() {
        let mut state = State::default();
        let mut view = simulator(state.view());

        assert!(view.click(selector::id(ID::Name.as_str())).is_ok());
        view.typewrite("ซันมินิมาร์ท");

        assert!(view.click(selector::id(ID::Recipient.as_str())).is_ok());
        view.typewrite("ผู้จัดการร้าน");

        view.into_messages().for_each(|message| {
            println!("{:?}", message);
            state.update(message)
        });
        assert_eq!(state.address.name, "ซันมินิมาร์ท");
        assert_eq!(state.address.recipient, "ผู้จัดการร้าน");
    }
}

use iced::{
    Element,
    Length::Fill,
    widget::{Id, column, combo_box, container, row, text, text_input},
};

use crate::geo_data::GEO_DATA;

enum ID {
    Name,
    Recipient,
    Address,
}

impl Into<Id> for ID {
    fn into(self) -> Id {
        match self {
            ID::Name => Id::new("name_text_input"),
            ID::Recipient => Id::new("recipient_text_input"),
            ID::Address => Id::new("address_text_input"),
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
    OnAddressInput(String),
    OnProvinceSelect(String),
    OnProvinceInput(String),
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::OnNameInput(name) => self.address.name = name,
            Message::OnRecipientInput(recipient) => self.address.recipient = recipient,
            Message::OnAddressInput(address) => self.address.address = address,
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
                    .id(ID::Name)
                    .on_input(Message::OnNameInput)
            ],
            row![
                text("ถึง"),
                text_input("", self.address.recipient.as_str())
                    .id(ID::Recipient)
                    .on_input(Message::OnRecipientInput)
            ],
            row![
                text("ที่อยู่"),
                text_input("", self.address.address.as_str())
                    .id(ID::Address)
                    .on_input(Message::OnAddressInput)
            ],
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
    use super::{ID, Message, State};
    use iced_test::{selector, simulator};

    #[test]
    fn name_text_input() {
        let input = "ซันมินิมาร์ท";
        let id = ID::Name;
        let mut state = State::default();
        let mut view = simulator(state.view());

        assert!(view.click(selector::id(id)).is_ok());
        view.typewrite(input);

        view.into_messages()
            .for_each(|message| state.update(message));
        assert_eq!(state.address.name, input);
    }

    #[test]
    fn recipient_text_input() {
        let input = "ผู้จัดการร้าน";
        let id = ID::Recipient;
        let mut state = State::default();
        let mut view = simulator(state.view());

        assert!(view.click(selector::id(id)).is_ok());
        view.typewrite(input);

        view.into_messages()
            .for_each(|message| state.update(message));
        assert_eq!(state.address.recipient, input);
    }

    #[test]
    fn address_text_input() {
        let input = "82/3 ม.7";
        let id = ID::Address;
        let mut state = State::default();
        let mut view = simulator(state.view());

        assert!(view.click(selector::id(id)).is_ok());
        view.typewrite(input);

        view.into_messages()
            .for_each(|message| state.update(message));
        assert_eq!(state.address.address, input);
    }

    // TODO: `iced_test` not yet support `combo_box` (no Id yet)
    #[test]
    fn province_combo_box() {
        let mut state = State::default();

        let inputs = ["ภ", "เ", "ก", "ต", "จ"];
        let results = [true, true, true, true, false];
        inputs
            .iter()
            .zip(results.iter())
            .for_each(|(input, &result)| {
                state.update(Message::OnProvinceInput(input.to_string()));
                assert_eq!(
                    state.provinces.options().contains(&"ภูเก็ต".to_string()),
                    result
                );
            });

        let input = "ภูเก็ต";
        state.update(Message::OnProvinceSelect(input.to_string()));
        assert_eq!(state.address.province, Some(input.to_string()));
    }
}

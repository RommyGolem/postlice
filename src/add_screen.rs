use iced::{
    Element,
    Length::Fill,
    widget::{Id, column, combo_box, container, row, text, text_input},
};

use crate::geo_data::{District, GEO_DATA, Province};

enum ID {
    Name,
    Recipient,
    Address,
}

impl From<ID> for Id {
    fn from(id: ID) -> Self {
        match id {
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
    districts: combo_box::State<String>,
    sub_districts: combo_box::State<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            address: Address::default(),
            provinces: combo_box::State::with_selection(Vec::new(), None),
            districts: combo_box::State::with_selection(Vec::new(), None),
            sub_districts: combo_box::State::with_selection(Vec::new(), None),
        }
    }
}

#[derive(Default, Clone)]
pub struct Address {
    name: String,
    recipient: String,
    address: String,
    province: (Option<String>, Province),
    district: (Option<String>, District),
    sub_district: Option<String>,
    postal_code: String,
    districts: combo_box::State<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    OnNameInput(String),
    OnRecipientInput(String),
    OnAddressInput(String),

    OnProvinceSelect(String),
    OnProvinceInput(String),

    OnDistrictSelect(String),
    OnDistrictInput(String),

    OnSubDistrictSelect(String),
    OnSubDistrictInput(String),
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::OnNameInput(name) => self.address.name = name,
            Message::OnRecipientInput(recipient) => self.address.recipient = recipient,
            Message::OnAddressInput(address) => self.address.address = address,

            Message::OnProvinceSelect(select) => {
                self.address.province = (
                    Some(select.clone()),
                    match GEO_DATA.iter().find(|province| province.name == select) {
                        Some(province) => province.clone(),
                        None => Province::default(),
                    },
                );
                self.address.districts = combo_box::State::with_selection(
                    self.address
                        .province
                        .1
                        .districts
                        .iter()
                        .map(|district| district.name.clone())
                        .collect(),
                    None,
                );
                self.address.district = (None, District::default());
                self.address.sub_district = None;
            }
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

            Message::OnDistrictSelect(select) => {
                self.address.district = (
                    Some(select.clone()),
                    match self
                        .address
                        .province
                        .1
                        .districts
                        .iter()
                        .find(|district| district.name == select)
                    {
                        Some(district) => district.clone(),
                        None => District::default(),
                    },
                );

                self.address.postal_code = self.address.district.1.postal_code.to_string();
            }
            Message::OnDistrictInput(filter) => {
                self.districts = combo_box::State::with_selection(
                    self.address
                        .province
                        .1
                        .districts
                        .iter()
                        .map(|district| district.name.clone())
                        .filter(|district| district.contains(&filter))
                        .collect(),
                    Some(&filter),
                )
            }
            Message::OnSubDistrictInput(filter) => {
                self.sub_districts = combo_box::State::with_selection(
                    self.address
                        .district
                        .1
                        .sub_districts
                        .iter()
                        .filter(|sub_district| sub_district.contains(&filter))
                        .cloned()
                        .collect(),
                    Some(&filter),
                )
            }
            Message::OnSubDistrictSelect(sub_district) => {
                self.address.sub_district = Some(sub_district);
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
                    self.address.province.0.as_ref(),
                    Message::OnProvinceSelect
                )
                .on_input(Message::OnProvinceInput)
            ],
            row![
                text("อำเภอ"),
                combo_box(
                    &self.districts,
                    "เลือกอำเภอ",
                    self.address.district.0.as_ref(),
                    Message::OnDistrictSelect
                )
                .on_input(Message::OnDistrictInput)
            ],
            row![
                text("ตำบล"),
                combo_box(
                    &self.sub_districts,
                    "เลือกตำบล",
                    self.address.sub_district.as_ref(),
                    Message::OnSubDistrictSelect
                )
                .on_input(Message::OnSubDistrictInput)
            ],
        ])
        .center(Fill)
        .into()
    }
}

#[cfg(test)]
mod test {
    use crate::geo_data::District;

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
        assert_eq!(state.address.province.0, Some(input.to_string()));
        assert_eq!(state.address.district, (None, District::default()));
    }

    #[test]
    fn district_combo_box() {
        let mut state = State::default();
        state.update(Message::OnProvinceSelect("ภูเก็ต".to_string()));

        let inputs = ["เ", "ม", "อ", "ง", "จ"];
        let results = [true, true, true, true, false];
        inputs
            .iter()
            .zip(results.iter())
            .for_each(|(input, &result)| {
                state.update(Message::OnDistrictInput(input.to_string()));
                assert_eq!(
                    state.districts.options().contains(&"เมืองภูเก็ต".to_string()),
                    result
                );
            });

        let input = "เมืองภูเก็ต";
        state.update(Message::OnDistrictSelect(input.to_string()));
        assert_eq!(state.address.district.0, Some(input.to_string()));
        assert_eq!(state.address.postal_code, "83000".to_string());
    }

    #[test]
    fn sub_district_combo_box() {
        let mut state = State::default();
        state.update(Message::OnProvinceSelect("ภูเก็ต".to_string()));
        state.update(Message::OnDistrictSelect("เมืองภูเก็ต".to_string()));

        let inputs = ["ว", "ช", "ต", "จ"];
        let results = [true, true, true, false];
        inputs
            .iter()
            .zip(results.iter())
            .for_each(|(input, &result)| {
                state.update(Message::OnSubDistrictInput(input.to_string()));
                assert_eq!(
                    state.sub_districts.options().contains(&"วิชิต".to_string()),
                    result
                );
            });

        let input = "วิชิต";
        state.update(Message::OnSubDistrictSelect(input.to_string()));
        assert_eq!(state.address.sub_district, Some(input.to_string()));
    }
}

use crate::geo_data::{District, GEO_DATA, Province};
use iced::{
    Element, Length, Task,
    widget::{Text, column, combo_box, container, row, text, text_input},
};

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

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    GotoHome,

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

fn labeled_text_input<'a>(
    label: &'a str,
    value: &'a str,
    on_input: &'a dyn Fn(String) -> Message,
) -> Element<'a, Message> {
    row![
        text(label).width(Length::Fill),
        text_input("", value)
            .on_input(on_input)
            .width(Length::FillPortion(6))
    ]
    .into()
}

fn labeled_combo_box<'a, F, G>(
    label: &'a str,
    state: &'a combo_box::State<String>,
    selection: Option<&'a String>,
    on_selected: F,
    on_input: G,
) -> Element<'a, Message>
where
    F: 'static + Fn(String) -> Message,
    G: 'static + Fn(String) -> Message,
{
    row![
        text(format!("{}:", label)).width(Length::Fill),
        combo_box(state, &format!("เลือก{}", label), selection, on_selected)
            .on_input(on_input)
            .width(Length::FillPortion(6))
    ]
    .into()
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<crate::Message> {
        let mut tasks = Vec::new();
        match message {
            Message::GotoHome => tasks.push(Task::done(crate::Message::GotoHome)),
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

        Task::batch(tasks)
    }

    pub fn view(&self) -> Element<'_, Message> {
        let name_text_input = labeled_text_input("ชื่อ:", &self.address.name, &Message::OnNameInput);

        let recipient_text_input =
            labeled_text_input("ถึง:", &self.address.recipient, &Message::OnRecipientInput);

        let address_text_input =
            labeled_text_input("ที่อยู่:", &self.address.address, &Message::OnAddressInput);

        let province_combo_box = labeled_combo_box(
            "จังหวัด",
            &self.provinces,
            self.address.province.0.as_ref(),
            Message::OnProvinceSelect,
            Message::OnProvinceInput,
        );

        let district_combo_box = labeled_combo_box(
            "อำเภอ",
            &self.districts,
            self.address.district.0.as_ref(),
            Message::OnDistrictSelect,
            Message::OnDistrictInput,
        );

        let sub_district_combo_box = labeled_combo_box(
            "ตำบล",
            &self.sub_districts,
            self.address.sub_district.as_ref(),
            Message::OnSubDistrictSelect,
            Message::OnSubDistrictInput,
        );

        let output_line_one: Text =
            text(format!("{}{}", self.address.recipient, self.address.name));
        let output_line_two: Text = text(self.address.address.clone());
        let output_line_three: Text = text(
            match (
                self.address.province.0 == Some("กรุงเทพมหานคร".to_string()),
                self.address.sub_district.clone(),
                self.address.district.0.clone(),
            ) {
                (true, Some(sub_district), Some(district)) => {
                    format!("แขวง{} เขต{}", sub_district, district)
                }
                (true, Some(sub_district), None) => {
                    format!("แขวง{}", sub_district)
                }
                (true, None, Some(district)) => {
                    format!("เขต{}", district)
                }
                (false, Some(sub_district), Some(district)) => {
                    format!("ตำบล{} อำเภอ{}", sub_district, district)
                }
                (false, Some(sub_district), None) => {
                    format!("ตำบล{}", sub_district)
                }
                (false, None, Some(district)) => {
                    format!("อำเภอ{}", district)
                }
                (_, _, _) => "".to_string(),
            },
        );
        let output_line_four: Text = text(
            match (
                self.address.province.0 == Some("กรุงเทพมหานคร".to_string()),
                self.address.province.0.clone(),
            ) {
                (true, _) => format!("กรุงเทพมหานคร {}", self.address.postal_code),
                (_, Some(district)) => format!("จังหวัด{} {}", district, self.address.postal_code),
                (_, _) => "".to_string(),
            },
        );

        container(
            row![
                container(
                    column![
                        name_text_input,
                        recipient_text_input,
                        address_text_input,
                        province_combo_box,
                        district_combo_box,
                        sub_district_combo_box,
                    ]
                    .width(Length::Fill)
                    .padding(10)
                    .spacing(10)
                )
                .style(|_| container::bordered_box(&iced::Theme::Light)),
                container(row![
                    column![text("เรียน")].width(Length::Fill).padding(10),
                    column![
                        output_line_one,
                        output_line_two,
                        output_line_three,
                        output_line_four
                    ]
                    .width(Length::FillPortion(6))
                    .padding(10)
                    .spacing(10)
                ])
                .style(|_| container::bordered_box(&iced::Theme::Light)),
            ]
            .spacing(50)
            .padding(50),
        )
        .center(Length::Fill)
        .into()
    }
}

#[cfg(test)]
mod test {
    use super::{Message, State};
    use crate::{geo_data::District, home_screen};

    #[test]
    fn back_button() {
        let mut state = crate::State {
            screen: crate::Screen::Add(Box::from(State::default())),
        };

        let task = state.update(crate::Message::AddScreen(Message::GotoHome));
        let _ = task.map(|message| assert_eq!(message, crate::Message::GotoHome));
        let _ = state.update(crate::Message::GotoHome);
        assert!(matches!(
            state.screen,
            crate::Screen::Home(home_screen::State)
        ));
    }

    #[test]
    fn name_text_input() {
        let input = "ซันมินิมาร์ท";
        let mut state = State::default();

        let _ = state.update(Message::OnNameInput(input.to_string()));
        assert_eq!(state.address.name, input);
    }

    #[test]
    fn recipient_text_input() {
        let input = "ผู้จัดการร้าน";
        let mut state = State::default();

        let _ = state.update(Message::OnRecipientInput(input.to_string()));
        assert_eq!(state.address.recipient, input);
    }

    #[test]
    fn address_text_input() {
        let input = "82/3 ม.7";
        let mut state = State::default();

        let _ = state.update(Message::OnAddressInput(input.to_string()));
        assert_eq!(state.address.address, input);
    }

    #[test]
    fn province_combo_box() {
        let mut state = State::default();

        let inputs = ["ภ", "เ", "ก", "ต", "จ"];
        let results = [true, true, true, true, false];
        inputs
            .iter()
            .zip(results.iter())
            .for_each(|(input, &result)| {
                let _ = state.update(Message::OnProvinceInput(input.to_string()));
                assert_eq!(
                    state.provinces.options().contains(&"ภูเก็ต".to_string()),
                    result
                );
            });

        let input = "ภูเก็ต";
        let _ = state.update(Message::OnProvinceSelect(input.to_string()));
        assert_eq!(state.address.province.0, Some(input.to_string()));
        assert_eq!(state.address.district, (None, District::default()));
    }

    #[test]
    fn district_combo_box() {
        let mut state = State::default();
        let _ = state.update(Message::OnProvinceSelect("ภูเก็ต".to_string()));

        let inputs = ["เ", "ม", "อ", "ง", "จ"];
        let results = [true, true, true, true, false];
        inputs
            .iter()
            .zip(results.iter())
            .for_each(|(input, &result)| {
                let _ = state.update(Message::OnDistrictInput(input.to_string()));
                assert_eq!(
                    state.districts.options().contains(&"เมืองภูเก็ต".to_string()),
                    result
                );
            });

        let input = "เมืองภูเก็ต";
        let _ = state.update(Message::OnDistrictSelect(input.to_string()));
        assert_eq!(state.address.district.0, Some(input.to_string()));
        assert_eq!(state.address.postal_code, "83000".to_string());
    }

    #[test]
    fn sub_district_combo_box() {
        let mut state = State::default();
        let _ = state.update(Message::OnProvinceSelect("ภูเก็ต".to_string()));
        let _ = state.update(Message::OnDistrictSelect("เมืองภูเก็ต".to_string()));

        let inputs = ["ว", "ช", "ต", "จ"];
        let results = [true, true, true, false];
        inputs
            .iter()
            .zip(results.iter())
            .for_each(|(input, &result)| {
                let _ = state.update(Message::OnSubDistrictInput(input.to_string()));
                assert_eq!(
                    state.sub_districts.options().contains(&"วิชิต".to_string()),
                    result
                );
            });

        let input = "วิชิต";
        let _ = state.update(Message::OnSubDistrictSelect(input.to_string()));
        assert_eq!(state.address.sub_district, Some(input.to_string()));
    }
}

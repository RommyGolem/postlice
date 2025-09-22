use iced::{
    self, Element, Fill, Subscription, keyboard,
    widget::{column, combo_box, container, row, text, text_input},
};

use postlice::{GEO_DATA, Province};

fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .subscription(subscription)
        .run()
}
struct State {
    count: i64,
    address: Address,
    provinces: combo_box::State<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            count: i64::default(),
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

#[derive(Default)]
struct Address {
    name: String,
    recipient: String,
    address: String,
    sub_district: Option<String>,
    district: Option<String>,
    province: Option<String>,
    postal_code: String,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    OnProvinceSelect(String),
    OnProvinceInput(String),
    // OnProVinceHover(String),
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Increment => state.count += 1,
        Message::Decrement => state.count -= 1,
        Message::OnProvinceSelect(province) => state.address.province = Some(province),
        Message::OnProvinceInput(filter) => {
            state.provinces = combo_box::State::with_selection(
                (*GEO_DATA
                    .iter()
                    .map(|province| province.name.clone())
                    .filter(|province| province.contains(&filter))
                    .collect::<Vec<String>>())
                .to_vec(),
                Some(&filter),
            )
        } // Message::OnProVinceHover(province) => state.address.province = Some(province),
    }
}

fn view(state: &State) -> Element<'_, Message> {
    container(column![
        row![text("ชื่อ"), text_input("", state.address.name.as_str())],
        row![text("ถึง"), text_input("", state.address.recipient.as_str())],
        row![text("ที่อยู่"), text_input("", state.address.address.as_str())],
        row![
            text("จังหวัด"),
            combo_box(
                &state.provinces,
                "เลือกจังหวัด",
                state.address.province.as_ref(),
                Message::OnProvinceSelect
            )
            .on_input(Message::OnProvinceInput)
        ],
    ])
    .center(Fill)
    .into()
}

fn subscription(_state: &State) -> Subscription<Message> {
    keyboard::on_key_press(|key, _modifier| match key {
        keyboard::Key::Named(keyboard::key::Named::ArrowUp) => Some(Message::Increment),
        keyboard::Key::Named(keyboard::key::Named::ArrowDown) => Some(Message::Decrement),
        _ => None,
    })
}

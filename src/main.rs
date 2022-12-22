use iced::{alignment, text_input, Column, Sandbox, Settings, Text, TextInput};
use iced_aw::split;

use serde::Deserializer;
use serde_json::Value;

fn main() {
    JsonViewer::run(Settings::default()).expect("run ui");
}

#[derive(Clone, Debug, Default)]
struct JsonViewer {
    filename: String,
    json: Option<Value>,

    query: String,
    json_result: Option<Value>,

    filename_state: text_input::State,
    query_state: text_input::State,
    split_state: split::State,
}

#[derive(Clone, Debug)]
enum Message {
    Filename(String),
    Query(String),
    Resize(u16),
}

impl Sandbox for JsonViewer {
    type Message = Message;

    fn new() -> Self {
        let query = String::new();
        let filename = String::from("./sample.json");
        let json = std::fs::read_to_string(&filename).unwrap_or_default();
        let json = serde_json::from_str(&json).ok();

        JsonViewer {
            filename,
            json,
            query,

            ..Default::default()
        }
    }

    fn title(&self) -> String {
        let mut t = String::from("Json Viewer");
        if self.json.is_none() {
            t += " - ";
            t += &self.filename;
        }
        t
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Filename(filename) => {
                let json = std::fs::read_to_string(&filename).unwrap_or_default();
                let json = serde_json::from_str(&json).ok();
                self.json = json;
                self.filename = filename;

                if let Some(json) = &self.json {
                    self.json_result = jql::walker(json, &self.query).ok()
                } else {
                    self.json_result = None;
                }
            }
            Message::Resize(n) => {
                eprintln!("Resize to {n}");
            }
            Message::Query(query) => {
                self.query = query;
                if let Some(json) = &self.json {
                    self.json_result = jql::walker(json, &self.query).ok()
                } else {
                    self.json_result = None;
                }
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        use iced_aw::Split;

        let fileselector = TextInput::new(
            &mut self.filename_state,
            "json file to open",
            &self.filename,
            Message::Filename,
        );

        let json_view = if let Some(json) = &self.json {
            Text::new(json.to_string())
        } else {
            Text::new("no json found in file").horizontal_alignment(alignment::Horizontal::Center)
        };

        let json_view = iced::Container::new(json_view).center_x().center_y();

        let left = Column::new().push(fileselector).push(json_view);

        let query = TextInput::new(
            &mut self.query_state,
            "json query",
            &self.query,
            Message::Query,
        );

        let json_view = if let Some(json) = &self.json_result {
            Text::new(json.to_string())
        } else {
            Text::new("no json found in file").horizontal_alignment(alignment::Horizontal::Center)
        };

        let right = Column::new().push(query).push(json_view);


        let s = Split::new(&mut self.split_state, left, right, Message::Resize);

        s.into()
    }
}

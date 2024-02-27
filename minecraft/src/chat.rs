use json::JsonValue;
use serde::{Serialize, Deserialize};

use crate::server::server_status::{Players, SamplePlayers, ServerStatus, ServerVersion};
#[derive(Serialize, Deserialize, Debug)]
pub enum Chat {
    Text {
        text: String,
        style: ChatStyle,
        with: Option<Box<Chat>>,
    },
    Translatable {
        translate: String,
        style: ChatStyle,
        with: Option<Box<Chat>>,
    },
    Keybind {
        keybind: String,
        style: ChatStyle,
        with: Option<Box<Chat>>,
    },
    Score {
        score: Score,
        style: ChatStyle,
        with: Option<Box<Chat>>,
    },
    Selector {
        selector: String,
        separator: Option<Box<Chat>>,
        style: ChatStyle,
        with: Option<Box<Chat>>,
    },
    Nbt {
        interpret: Option<bool>,
        separator: Option<Box<Chat>>,
        nbt_type: ChatNbtType,
        style: ChatStyle,
        with: Option<Box<Chat>>,
    },
}

impl From<String> for Chat {
    fn from(value: String) -> Chat {
        Chat::Text {
            text: value,
            style: ChatStyle {
                color: None,
                styles: Styles {
                    bold: false,
                    italic: false,
                    underlined: false,
                    strikethrough: false,
                    obfuscated: false,
                },
                font: None,
                insertion: None,
                click_event: None,
                hover_event: None,
            },
            with: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum ChatNbtType {
    Block(String),
    Entity(String),
    Storage(String),
}

#[derive(Serialize, Deserialize, Debug)]
struct Score {
    name: String,
    objective: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatStyle {
    color: Option<String>,
    styles: Styles,
    font: Option<String>,
    insertion: Option<String>,
    click_event: Option<ClickEvent>,
    hover_event: Option<HoverEvent>,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
struct Styles {
    bold: bool,
    italic: bool,
    underlined: bool,
    strikethrough: bool,
    obfuscated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClickEvent {
    OpenUrl(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(String),
    CopyToClipboard(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HoverEvent {
    ShowText(Box<Chat>),
    ShowItem {
        id: String,
        count: i32,
        tag: Option<String>,
    },
    ShowEntity {
        entity_type: String,
        id: String,
        name: Option<String>,
    },
}

#[test]
fn test_chat_ser() {
    serde_json::to_string(&Chat::from("hi".to_string())).unwrap();
}

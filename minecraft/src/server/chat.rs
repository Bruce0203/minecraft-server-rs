use bitflags::bitflags;
use json::JsonValue;

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
                styles: Styles::None,
                font: None,
                insertion: None,
                click_event: None,
                hover_event: None,
            },
            with: None,
        }
    }
}

enum ChatNbtType {
    Block(String),
    Entity(String),
    Storage(String),
}

struct Score {
    name: String,
    objective: String,
}

pub struct ChatStyle {
    color: Option<String>,
    styles: Styles,
    font: Option<String>,
    insertion: Option<String>,
    click_event: Option<Box<ClickEvent>>,
    hover_event: Option<Box<HoverEvent>>,
}

bitflags! {
     #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Styles: u8 {
        const Bold = 0b_0000_0001;
        const Italic = 0b_0000_0010;
        const Underlined = 0b_0000_0100;
        const Strikethrough = 0b_0000_1000;
        const Obfuscated = 0b_0001_0000;
        const None = 0;
    }
}

pub enum ClickEvent {
    OpenUrl(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(String),
    CopyToClipboard(String),
}

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

impl From<&Chat> for JsonValue {
    fn from(value: &Chat) -> Self {
        let mut data = json::JsonValue::new_object();
        match value {
            Chat::Text { text, style, with } => {
                data["text"] = text.as_str().into();
                encode_style_and_with(&mut data, style, with);
            }
            Chat::Translatable {
                translate,
                style,
                with,
            } => {
                data["translate"] = translate.as_str().into();
                encode_style_and_with(&mut data, style, with);
            }
            Chat::Keybind {
                keybind,
                style,
                with,
            } => todo!(),
            Chat::Score { score, style, with } => todo!(),
            Chat::Selector {
                selector,
                separator,
                style,
                with,
            } => todo!(),
            Chat::Nbt {
                interpret,
                separator,
                nbt_type,
                style,
                with,
            } => todo!(),
        }
        data
    }
}

fn encode_style_and_with(data: &mut JsonValue, style: &ChatStyle, with: &Option<Box<Chat>>) {
    style.encode_chat_style_to_json(data);
    if let Some(with) = with {
        data["with"] = (&**with).into();
    }
}

impl ChatStyle {
    fn encode_chat_style_to_json(&self, data: &mut JsonValue) {
        if let Some(color) = &self.color {
            data["color"] = color.as_str().into();
        }
        self.styles.encode_style(data);
        if let Some(font) = &self.font {
            data["font"] = font.as_str().into();
        }
        if let Some(insertion) = &self.insertion {
            data["insertion"] = insertion.as_str().into();
        }
        if let Some(click_event) = &self.click_event {
            data["click_event"] = (&**click_event).into();
        }
        if let Some(hover_event) = &self.hover_event {
            data["hover_event"] = (&**hover_event).into();
        }
    }
}

impl Styles {
    fn encode_style(&self, data: &mut JsonValue) {
        if *self & Styles::Bold == Styles::Bold {
            data["bold"] = true.into();
        }
        if *self & Styles::Italic == Styles::Italic {
            data["italic"] = true.into();
        }
        if *self & Styles::Underlined == Styles::Underlined {
            data["rnderlined"] = true.into();
        }
        if *self & Styles::Strikethrough == Styles::Strikethrough {
            data["strikethrough"] = true.into();
        }
        if *self & Styles::Obfuscated == Styles::Obfuscated {
            data["obfuscated"] = true.into();
        }
    }
}

impl From<&ClickEvent> for JsonValue {
    fn from(value: &ClickEvent) -> Self {
        let mut data = json::JsonValue::new_object();
        match value {
            ClickEvent::OpenUrl(value) => data["open_url"] = value.as_str().into(),
            ClickEvent::RunCommand(value) => data["run_command"] = value.as_str().into(),
            ClickEvent::SuggestCommand(value) => data["suggest_command"] = value.as_str().into(),
            ClickEvent::ChangePage(value) => data["change_page"] = value.as_str().into(),
            ClickEvent::CopyToClipboard(value) => data["copy_to_clipboard"] = value.as_str().into(),
        }
        data
    }
}

impl From<&HoverEvent> for JsonValue {
    fn from(value: &HoverEvent) -> Self {
        todo!()
    }
}

#[test]
fn chat_ser() {
    let chat = Chat::from("hi".to_string());
    let result: JsonValue = (&chat).into();
}

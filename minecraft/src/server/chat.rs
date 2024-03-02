use std::ops::Deref;

use bitflags::bitflags;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Debug)]
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

#[derive(Debug)]
enum ChatNbtType {
    Block(String),
    Entity(String),
    Storage(String),
}

#[derive(Debug)]
struct Score {
    name: String,
    objective: String,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ClickEvent {
    OpenUrl(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(String),
    CopyToClipboard(String),
}

#[derive(Debug)]
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

impl Serialize for Chat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Chat::Text { text, style, with } => {
                let mut state = serializer.serialize_struct("chat", 11)?;
                state.serialize_field("text", text)?;
                style.serialize_chat_style(state, with)
            }
            Chat::Translatable {
                translate,
                style,
                with,
            } => {
                let mut state = serializer.serialize_struct("translate", 11)?;
                state.serialize_field("translate", translate)?;
                style.serialize_chat_style(state, with)
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
    }
}

impl Serialize for ClickEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("clickEvent", 2)?;
        match self {
            ClickEvent::OpenUrl(value) => {
                state.serialize_field("action", "open_url")?;
                state.serialize_field("value", value)?;
            }
            ClickEvent::RunCommand(value) => {
                state.serialize_field("action", "run_command")?;
                state.serialize_field("value", value)?;
            }
            ClickEvent::SuggestCommand(value) => {
                state.serialize_field("action", "suggest_command")?;
                state.serialize_field("value", value)?;
            }
            ClickEvent::ChangePage(value) => {
                state.serialize_field("action", "change_page")?;
                state.serialize_field("value", value)?;
            }
            ClickEvent::CopyToClipboard(value) => {
                state.serialize_field("action", "copy_to_clipboard")?;
                state.serialize_field("value", value)?;
            }
        }
        state.end()
    }
}

impl Serialize for HoverEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("hoverEvent", 2)?;
        match self {
            HoverEvent::ShowText(text) => {
                state.serialize_field("action", "show_text")?;
                state.serialize_field("contents", text)?;
            }
            HoverEvent::ShowItem { id, count, tag } => {
                state.serialize_field("action", "show_item")?;
                state.serialize_field("id", id)?;
                state.serialize_field("count", count)?;
                state.serialize_field("tag", tag)?;
            }
            HoverEvent::ShowEntity {
                entity_type,
                id,
                name,
            } => {
                state.serialize_field("type", entity_type)?;
                state.serialize_field("id", id)?;
                state.serialize_field("name", name)?;
            }
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for Chat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

impl ChatStyle {
    fn serialize_chat_style<S: SerializeStruct>(
        &self,
        mut state: S,
        with: &Option<Box<Chat>>,
    ) -> Result<S::Ok, S::Error> {
        let style = self;
        if let Some(style) = &style.color {
            state.serialize_field("color", style)?;
        } else {
            state.skip_field("color")?;
        }
        let styles = style.styles;
        if styles & Styles::Bold == Styles::Bold {
            state.serialize_field("bold", &true)?;
        } else {
            state.skip_field("bold")?;
        }
        if styles & Styles::Italic == Styles::Italic {
            state.serialize_field("italic", &true)?;
        } else {
            state.skip_field("italic")?;
        }
        if styles & Styles::Underlined == Styles::Underlined {
            state.serialize_field("underlined", &true)?;
        } else {
            state.skip_field("underlined")?;
        }
        if styles & Styles::Strikethrough == Styles::Strikethrough {
            state.serialize_field("strikethrough", &true)?;
        } else {
            state.skip_field("strikethrough")?;
        }
        if styles & Styles::Obfuscated == Styles::Obfuscated {
            state.serialize_field("obfuscated", &true)?;
        } else {
            state.skip_field("obfuscated")?;
        }
        if let Some(font) = &style.font {
            state.serialize_field("font", font)?;
        } else {
            state.skip_field("font")?;
        }
        if let Some(insertion) = &style.insertion {
            state.serialize_field("insertion", &insertion)?;
        }
        if let Some(click_event) = &style.click_event {
            state.serialize_field("clickEvent", click_event)?;
        } else {
            state.skip_field("clickEvent")?;
        }
        if let Some(hover_event) = &style.hover_event {
            state.serialize_field("hoverEvent", hover_event)?;
        } else {
            state.skip_field("hoverEvent")?;
        }
        if let Some(with) = with {
            state.serialize_field("with", with.deref())?;
        }
        state.end()
    }
}

impl Serialize for ChatStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("style", 10)?;
        self.serialize_chat_style(state, &None)
    }
}

impl<'de> Deserialize<'de> for ChatStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        todo!()
    }
}

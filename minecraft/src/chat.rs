use bitflags::bitflags;

pub struct Chat {
    inner: ChatExt,
    style: ChatStyle,
    with: Option<Box<Chat>>,
}

enum ChatExt {
    Text {
        text: String,
    },
    Translatable {
        translate: String,
    },
    Keybind {
        keybind: String,
    },
    Score {
        score: Score,
    },
    Selector {
        selector: String,
        separator: Option<Box<Chat>>,
    },
    Nbt {
        interpret: Option<bool>,
        separator: Option<Box<Chat>>,
        nbt_type: ChatNbtType,
    },
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
    click_event: Option<ClickEvent>,
    hover_event: Option<HoverEvent>,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Styles: u8 {
        const None = 0;
        const Bold = 0b_0000_0001;
        const Italic = 0b_0000_0010;
        const Underlined = 0b_0000_0100;
        const StrikeThrough = 0b_0000_1000;
        const Obfuscated = 0b_0001_0000;
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

impl From<&str> for Chat {
    fn from(value: &str) -> Self {
        Chat {
            inner: ChatExt::Text {
                text: String::from(value),
            },
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

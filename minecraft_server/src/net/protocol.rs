pub trait ProtocolId {
    const PROTOCOL_ID: i32;
}

pub trait Protocol {
    type Player;
    type Server;
}

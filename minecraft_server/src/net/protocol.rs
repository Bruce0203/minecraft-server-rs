pub trait Protocol {
    const PROTOCOL_ID: i32;
    type Player;
    type Server;
}

pub mod v1_20_4;

mod packet_handler;
mod packet_read_handler;
mod packet_writer;
pub mod prelude;
mod session_relay;

const LATEST_VERSION: &str = "1.20.4";
const LATEST_PROTOCOL_VERSION: i32 = 765;

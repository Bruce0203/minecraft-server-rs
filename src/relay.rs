use std::io::Read;

//Consumer of Socket read; should be registered as callback
trait Relay {}

//TODO: change to struct
trait RelaySession {
    fn read();
    fn set_protocol_id();
    fn get_protocol_id();
    fn set_compression_threshold();
    fn get_compression_threshold();
    fn set_encryption_enabled();
    fn is_encryption_enabled();
}

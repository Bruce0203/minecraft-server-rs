use bytes::BytesMut;

pub trait PacketEncoder {
    fn encode_packet_to_bytes(&self, bytes: &mut BytesMut);

    fn encode_packet(&self) -> BytesMut {
        let mut bytes = BytesMut::new();
        self.encode_packet_to_bytes(&mut bytes);
        bytes
    }
}

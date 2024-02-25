pub trait PacketHandler<T> {
    fn handle_packet(&self, system: &mut T);
}

use crate::server::slot::Slot;

pub struct SetContainerContent {
    window_id: u8,
    state_id: i32,
    count: i32,
    slot_data: Vec<Slot>,
    carried_item: Slot,
}

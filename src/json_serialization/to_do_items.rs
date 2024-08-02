use serde::Serialize;
use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::to_do::{to_do_factory, ItemTypes};
use crate::to_do::structs::base::Base;


#[derive(Serialize)]
pub struct ToDoItems{
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();
        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_array_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct)
            }
        }
        let pending_item_count = pending_array_buffer.len() as i8;
        let done_item_count = done_array_buffer.len() as i8;
        ToDoItems {
            pending_items: pending_array_buffer,
            done_items: done_array_buffer,
            pending_item_count,
            done_item_count
        }
    }

    pub fn get_state() -> ToDoItems {
        let state = read_file("./state.json").unwrap();
    let mut array_buffer = Vec::new();
    for (key, value) in state {
        let status = TaskStatus::from_string(value.to_string());
        let item: ItemTypes = to_do_factory(&key, status);
        array_buffer.push(item);
    }
    ToDoItems::new(array_buffer)
    }
}
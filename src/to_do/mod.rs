use structs::{done::Done, pending::Pending};

pub mod structs;

pub enum ItemTypes{
    Pending(Pending),
    Done(Done),
}


pub fn to_do_factory(item_type: &str, input_title: &str) -> Result<ItemTypes, &'static str>{
    if item_type == "pending"{
        let pending_item = Pending::new(input_title);
        Ok(ItemTypes::Pending(pending_item))
    }else if item_type == "done" {
        let done_item = Done::new(input_title);
        Ok(ItemTypes::Done(done_item))
    }else {
        Err("This is not accepted")
    }
}
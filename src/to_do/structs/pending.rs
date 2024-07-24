use super::{base::Base, traits::delete::Delete, traits::create::Create, traits::get::Get, traits::edit::Edit};

pub struct Pending{
    pub super_struct: Base
}

impl Pending {
    pub fn new(input_title: &str) -> Pending {
        let base = Base::new(input_title, "Pending");
        Pending{super_struct: base}
    }
}

impl Create for Pending {}
impl Get for Pending {}
impl Delete for Pending {}
impl Edit for Pending {}
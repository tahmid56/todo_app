use super::{base::Base, traits::delete::Delete, traits::create::Create, traits::get::Get, traits::edit::Edit};
use super::super::enums::TaskStatus;
pub struct Pending{
    pub super_struct: Base
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::PENDING
        };
        Pending{super_struct: base}
    }
}

impl Create for Pending {}
impl Get for Pending {}
impl Delete for Pending {}
impl Edit for Pending {}
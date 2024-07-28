use super::base::Base;
use super::traits::{get::Get, edit::Edit, delete::Delete};
use super::super::enums::TaskStatus;

pub struct Done{
    pub super_struct: Base
}

impl Done {
    pub fn new(input_title: &str) -> Done {
        let base = Base{
            title: input_title.to_string(),
            status: TaskStatus::DONE
        };
        Done{super_struct: base}
    }
}
impl Get for Done {}
impl Edit for Done {}
impl Delete for Done {}
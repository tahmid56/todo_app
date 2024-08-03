use actix_web::{HttpRequest, HttpResponse};
use serde_json::Map;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::read_file;
use crate::processes::process_input;
use crate::to_do::{to_do_factory, enums::TaskStatus}; 

pub async fn create(req: HttpRequest) -> HttpResponse {
    let state = read_file("./state.json");
    let title: String = req.match_info().get("title").unwrap().to_string();
    match state {
        Some(s)=> {
            let item = to_do_factory(title.as_str(), TaskStatus::PENDING);
            process_input(item, "create".to_string(), &s);
            
        },
        None => {
            let state = Map::new();
            process_input(to_do_factory(&title, TaskStatus::PENDING), "create".to_string(), &state);
        }
    }
    HttpResponse::Ok().json(ToDoItems::get_state())
}
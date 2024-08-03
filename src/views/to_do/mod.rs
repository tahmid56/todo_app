mod create;
mod get;
mod edit;
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn to_do_views_factory(app: &mut ServiceConfig){
    app.service(
        scope("v1/item")
        .route("get", get().to(get::get))
        .route("create/{title}", post().to(create::create))
        .route("edit", post().to(edit::edit))
    );
}
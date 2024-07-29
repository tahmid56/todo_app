mod create;
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn to_do_views_factory(app: &mut ServiceConfig){
    app.service(
        scope("v1/item")
        .route("create/{title}", get().to(create::create))
        // .route("create/{title}", post().to(create::create))
    );
}
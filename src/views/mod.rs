mod auth;
mod to_do;
mod app;
use auth::auth_view_factory;
use to_do::to_do_views_factory;
use actix_web::web::ServiceConfig;

pub fn views_factory(app: &mut ServiceConfig){
    auth_view_factory(app);
    to_do_views_factory(app);
    app::app_views_factory(app);
}
use actix_web::web::{ServiceConfig, get, scope};
mod login;
mod logout;


pub fn auth_view_factory(app: &mut ServiceConfig) {
    app.service(scope("v1/auth")
       .route("login", get().to(login::login))
       .route("logout", get().to(logout::logout))
    );
}
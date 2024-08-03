use actix_web::{http::Error, FromRequest};
use futures::future::{ok, Ready};



pub struct JwtToken{
    pub message: String,
}

impl FromRequest for JwtToken{
    type Error = Error;
    type Future = Ready<Result<JwtToken, Error>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req.headers().get("token"){
            Some(data) =>{
                let token = JwtToken{
                    message: data.to_str().unwrap().to_string()
                };
                ok(token)
            },
            None => {
                let token = JwtToken{
                    message: "nothing found".to_string()
                };
                ok(token)
            }
            
        }
    }
}
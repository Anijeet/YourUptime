use std::sync::{Arc, Mutex};

use poem::{ EndpointExt, Route, Server, get, listener::TcpListener, post};

use crate::{routes::{user::{sign_in, sign_up}, website::{create_website, get_website}}};
pub mod request_input;
pub mod request_output;
use store::store::Store;


pub mod routes;



#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
     let s = Arc::new(Mutex::new(Store::new().unwrap())); //here we added mutex and arc for not calling database on every route function like we are calling before in comented code
    let app = Route::new()
    .at("/status/:website_id", get(get_website))
    .at("/website", post(create_website))
    .at("/user/signup", post(sign_up))
    .at("/user/signin", post(sign_in))
    .data(s);

    //create a http  server that listens on port 3001
    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(app)
        .await
}
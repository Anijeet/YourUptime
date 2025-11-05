use poem::{ Route, Server, get, handler, listener::TcpListener, post, web::{Json, Path}};

use crate::{request_input::CreateWebsiteInput, request_output::CreateWebsiteOutput};
use crate::{request_input::CreateUserInput, request_output::CreateUserOutput};
use crate::{request_output::SigninUserOutput,request_output::GetWebsiteOutput};
pub mod request_input;
pub mod request_output;
use store::{models::{ website::Website}, store::Store};

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput>{
    let mut s = Store::new().unwrap();
    let website: Website = s.create_website(String::from("e6becd61-a838-409e-ae76-5f3a7b599de5"),data.url).unwrap();
    // let url = data.url;
    let response = CreateWebsiteOutput {
        id:website.id
    };

    Json(response)
}

#[handler]
fn get_website(Path(website_id): Path<String>) -> Json<GetWebsiteOutput> {
    let mut s = Store::new().unwrap();
    let website = s.get_website(website_id).unwrap();
    let response= GetWebsiteOutput{
        url:website.url
    };
    Json(response)
}

#[handler]
fn sign_up(Json(data): Json<CreateUserInput>)-> Json<CreateUserOutput>{
    let mut s = Store::new().unwrap();
    let user_id = s.sign_up(data.username, data.password).unwrap();
    let user = CreateUserOutput {
        id: user_id,
    };
    Json(CreateUserOutput { id: user.id })
}

#[handler]
fn sign_in(Json(data): Json<CreateUserInput>)-> Json<SigninUserOutput>{
    let mut s = Store::new().unwrap();
    let _exists = s.sign_in(data.username, data.password).unwrap(); //_exits mean if not used this variable then do _var for not getting warning
    let response = SigninUserOutput {
        jwt:String::from("Anijeet"),
    };
    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
    .at("/status/:website_id", get(get_website))
    .at("/website", post(create_website))
    .at("/user/signup", post(sign_up))
    .at("/user/signin", post(sign_in));


    //create a http  server that listens on port 3001
    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(app)
        .await
}
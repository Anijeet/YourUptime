use poem::{ Route, Server, get, handler, listener::TcpListener, post, web::{Json, Path}};

use crate::{request_input::CreateWebsiteInput, request_output::CreateWebsiteOutput};

pub mod request_input;
pub mod request_output;
use store::Store;

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput>{
    let s = Store{};
    let id = s.create_website();
    // let url = data.url;
    let response = CreateWebsiteOutput {
        id
    };

    Json(response)
}

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("hello: {}", website_id) //format concatenates strings like `hello ${website_id}`
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
    .at("/status/:website_id", get(get_website))
    .at("/website", post(create_website));

    //create a http  server that listens on port 3001
    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(app)
        .await
}
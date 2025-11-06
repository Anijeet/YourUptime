use std::sync::{Arc, Mutex};

use poem::{  handler, web::{Data, Json, Path}};

use crate::{request_input::CreateWebsiteInput, request_output::CreateWebsiteOutput};
use crate::{request_output::GetWebsiteOutput};
use store::{models::{ website::Website}, store::Store};




#[handler]
pub fn create_website(Json(data): Json<CreateWebsiteInput>, Data(s):Data<&Arc<Mutex<Store>>>) -> Json<CreateWebsiteOutput>{
    // let mut s = Store::new().unwrap();
     let mut locked_s = s.lock().unwrap();
    let website: Website = locked_s.create_website(String::from("e6becd61-a838-409e-ae76-5f3a7b599de5"),data.url).unwrap();
    // let url = data.url;
    let response = CreateWebsiteOutput {
        id:website.id
    };

    Json(response)
}

#[handler]
pub fn get_website(Path(website_id): Path<String>,Data(s):Data<&Arc<Mutex<Store>>>) -> Json<GetWebsiteOutput> {
    // let mut s = Store::new().unwrap();
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(website_id).unwrap();
    let response= GetWebsiteOutput{
        url:website.url
    };
    Json(response)
}

use std::sync::{Arc, Mutex};

use poem::{ handler,  web::{Data, Json}};
use store::store::Store;
use crate::{request_input::CreateUserInput, request_output::CreateUserOutput};
use crate::{request_output::SigninUserOutput};

#[handler]
pub fn sign_up(Json(data): Json<CreateUserInput>,Data(s):Data<&Arc<Mutex<Store>>>)-> Json<CreateUserOutput>{
    // let mut s = Store::new().unwrap();
     let mut locked_s = s.lock().unwrap();
    let user_id = locked_s.sign_up(data.username, data.password).unwrap();
    let user = CreateUserOutput {
        id: user_id,
    };
    Json(CreateUserOutput { id: user.id })
}

#[handler]
pub fn sign_in(Json(data): Json<CreateUserInput>,Data(s):Data<&Arc<Mutex<Store>>>)-> Json<SigninUserOutput>{
    // let mut s = Store::new().unwrap();
     let mut locked_s = s.lock().unwrap();
    let _exists = locked_s.sign_in(data.username, data.password).unwrap(); //_exits mean if not used this variable then do _var for not getting warning
    let response = SigninUserOutput {
        jwt:String::from("Anijeet"),
    };
    Json(response)
}
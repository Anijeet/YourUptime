use std::sync::{Arc, Mutex};

use crate::request_output::SigninUserOutput;
use crate::{request_input::CreateUserInput, request_output::CreateUserOutput};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
// use poem::Error;
use poem::{
    Error, handler,
    http::StatusCode,
    web::{Data, Json},
};
use store::{schema::website::user_id, store::Store};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[handler]
pub fn sign_up(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<CreateUserOutput>, Error> {
    // let mut s = Store::new().unwrap();
    let mut locked_s = s.lock().unwrap();
    let id = locked_s.sign_up(data.username, data.password).map_err(|_| Error::from_status(StatusCode::CONFLICT))?;
    let user = CreateUserOutput { id };
    Ok(Json(user))
}

#[handler]
pub fn sign_in(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<SigninUserOutput>, Error> {
    // let mut s = Store::new().unwrap();
    let mut locked_s = s.lock().unwrap();
    let result = locked_s.sign_in(data.username, data.password); //_exits mean if not used this variable then do _var for not getting warning

    match result {
        Ok(result) => {

            let my_claims = Claims{
                sub: result,
                exp: 11111111111
            };
            let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;

            let response = SigninUserOutput {
                jwt: token,
            };
            Ok(Json(response))
        }
        Err(e) => Err(Error::from_status(StatusCode::UNAUTHORIZED)),
    }
}

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct LoginCmdArgs {
    pub email_address: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginCmdArgsWrapper {
    pub payload: LoginCmdArgs,
}

#[derive(Debug, Deserialize)]
struct LoginEventRes {
    message: String,
    status: u32,
}

#[derive(Debug, Deserialize)]
struct LoginEventResWrapper {
    payload: LoginEventRes,
}

pub async fn login_handler(data: LoginCmdArgsWrapper) -> String {
    let args_json = to_value(&data).unwrap();

    // let res_data = invoke("login", args_json).await.as_string().unwrap();

    format!("{:?}", args_json.as_string()).into()
}

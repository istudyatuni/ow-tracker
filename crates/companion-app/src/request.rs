use reqwest::{StatusCode, Url};

use tracing::{debug, error, trace};
use uuid::Uuid;

use common::saves::Packed;
use common::server_models::*;

use crate::log::LogError;

fn server_url() -> Url {
    crate::SERVER_ADDRESS
        .parse()
        .expect("server url should be valid")
}

pub fn send_register(save: Vec<Packed>) -> Result<RegisterResponse, ()> {
    debug!("sending register request");
    let client = reqwest::blocking::Client::new();
    let Ok(resp) = client
        .post(
            server_url()
                .join("/api/register")
                .expect("url path should be valid"),
        )
        .json(&common::server_models::RegisterRequest { save })
        .send()
        .log_msg("failed to send register request")
    else {
        return Err(());
    };
    if resp.error_for_status_ref().is_err() {
        match resp.text() {
            Ok(text) => error!("error registering save: {text}"),
            Err(e) => error!("error registering save (failed to get response text: {e:?})"),
        }
        return Err(());
    }
    let Ok(resp) = resp
        .json::<common::server_models::RegisterResponse>()
        .log_msg("register response")
    else {
        return Err(());
    };

    Ok(resp)
}

pub fn send_register_update(id: Uuid, save: Vec<Packed>) -> Result<(), ()> {
    debug!("sending register update request");
    let client = reqwest::blocking::Client::new();
    let Ok(resp) = client
        .put(
            server_url()
                .join("/api/register")
                .expect("url path should be valid"),
        )
        .json(&common::server_models::UpdateRegisterRequest { id, save })
        .send()
        .log_msg("failed to send update request")
    else {
        return Err(());
    };
    if resp.error_for_status_ref().is_err() {
        match resp.text() {
            Ok(text) => error!("error updating save: {text}"),
            Err(e) => error!("error updating save (failed to get response text: {e:?})"),
        }
        return Err(());
    }
    if resp.status() == StatusCode::NOT_MODIFIED {
        trace!("save not modified");
    }

    Ok(())
}

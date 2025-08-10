use reqwest::blocking::{Client, Response};
use reqwest::{Method, Result as ReqResult, StatusCode, Url};

use serde::Serialize;
use serde::de::DeserializeOwned;
use tracing::{debug, error, trace};
use uuid::Uuid;

use common::saves::Packed;
use common::server_models::*;

use crate::config::LoadedConfig;

#[derive(Debug)]
pub struct Requester {
    client: Client,
    address: Url,
    new_address: Option<Url>,
}

impl Requester {
    pub fn new() -> Self {
        let client = Client::new();

        Self {
            client,
            address: crate::SERVER_ADDRESS
                .parse()
                .expect("server url should be valid"),
            new_address: None,
        }
    }
    fn address(&self) -> &Url {
        self.new_address.as_ref().unwrap_or(&self.address)
    }
    pub fn with_address(mut self, address: &str) -> Result<Self, String> {
        self.new_address.replace(
            address
                .parse()
                .map_err(|e| format!("failed to parse url: {e}"))?,
        );
        Ok(self)
    }
    fn get_json<Resp: DeserializeOwned>(&self, path: &str) -> ReqResult<Resp> {
        self.send_json_no_body(Method::GET, path)
    }
    fn get(&self, path: &str) -> ReqResult<Response> {
        self.send_no_body(Method::GET, path)
    }
    fn post_json<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
    ) -> ReqResult<Resp> {
        self.send_json(Method::POST, path, body)
    }
    #[expect(unused)]
    fn put_json<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
    ) -> ReqResult<Resp> {
        self.put(path, body)?.json()
    }
    fn put<Req: Serialize>(&self, path: &str, body: &Req) -> ReqResult<Response> {
        self.send_body(Method::PUT, path, body)
    }
    fn send_json<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: &Req,
    ) -> ReqResult<Resp> {
        self.send_body(method, path, body)?.json()
    }
    fn send_json_no_body<Resp: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
    ) -> ReqResult<Resp> {
        self.send_no_body(method, path)?.json()
    }
    fn send_body<Req: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: &Req,
    ) -> ReqResult<Response> {
        self.send(method, path, Some(body))
    }
    fn send_no_body(&self, method: Method, path: &str) -> ReqResult<Response> {
        self.send::<()>(method, path, None)
    }
    fn send<Req: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: Option<&Req>,
    ) -> ReqResult<Response> {
        trace!("sending {method} request to {path}");
        let mut req = self.client.request(
            method.clone(),
            self.address().join(path).expect("should be valid url path"),
        );
        if let Some(body) = body {
            req = req.json(body);
        }
        let resp = req.send()?;
        if let Err(e) = resp.error_for_status_ref() {
            let status = resp.status();
            match resp.text() {
                Ok(text) => error!("error sending {method} to {path} (status: {status}): {text}"),
                Err(e) => error!(
                    "error sending {method} to {path} (status: {status}, failed to get response text: {e:?})"
                ),
            }
            return Err(e);
        };
        Ok(resp)
    }
}

pub fn ping() -> Result<(), ()> {
    debug!("sending ping request");
    Requester::new()
        .get("/api/ping")
        .inspect_err(|e| error!("failed to ping: {e}"))
        .map(|_| ())
        .map_err(|_| ())
}

pub fn auth() -> Result<AuthResponse, ()> {
    debug!("sending auth request");
    Requester::new()
        .post_json("/api/auth", &AuthRequest { name: None })
        .inspect_err(|e| error!("failed to auth: {e}"))
        .map_err(|_| ())
}

// todo: pass name
pub fn send_register(key: Uuid, save: Vec<Packed>) -> Result<RegisterResponse, ()> {
    debug!("sending register request");
    Requester::new()
        .post_json("/api/register", &RegisterRequest { key, save })
        .inspect_err(|e| error!("failed to register save: {e}"))
        .map_err(|_| ())
}

pub fn send_register_update(id: Uuid, key: Uuid, save: Vec<Packed>) -> Result<(), ()> {
    debug!("sending register update request");
    let resp = Requester::new()
        .put("/api/register", &UpdateRegisterRequest { id, key, save })
        .inspect_err(|e| error!("failed to update save: {e}"))
        .map_err(|_| ())?;

    if resp.status() == StatusCode::NOT_MODIFIED {
        trace!("save not modified");
    }

    Ok(())
}

pub fn get_server_config(url: &str) -> Result<LoadedConfig, ()> {
    debug!("loading server config");
    Requester::new()
        .with_address(url)
        .inspect_err(|e| error!("invalid url for server config: {e}"))
        .map_err(|_| ())?
        .get_json("/config.json")
        .inspect_err(|e| error!("failed to get server config: {e}"))
        .map_err(|_| ())
}

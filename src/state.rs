use crate::https::HttpsClient;
use clap::ArgMatches;
use std::error::Error;
use serde_json::{to_string};
use axum::{
    response::{Response},
};
use axum::body::Body;
use hyper::Request;

use crate::create_https_client;
use crate::error::Error as RestError;
use crate::acls::AclDefinition;

type BoxResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Clone, Debug)]
pub struct State {
    pub client: HttpsClient,
    pub api: String
}

impl State {
    pub async fn new(opts: ArgMatches<'_>) -> BoxResult<Self> {
        // Set timeout
        let timeout: u64 = opts
            .value_of("timeout")
            .unwrap()
            .parse()
            .unwrap_or_else(|_| {
                eprintln!("Supplied timeout not in range, defaulting to 60");
                60
            });

        let client = create_https_client(timeout)?;
        let api = opts.value_of("api").unwrap().to_string();

        Ok(State {
            client,
            api
        })
    }

    pub async fn delete(&self, payload: AclDefinition) -> Result<Response<Body>, RestError> {
        let uri = format!("{}?{}", &self.api, payload.query_pairs());
//        let req = Request::builder()
//            .method("GET")
//            .uri(&uri)
//            .body(Body::empty())
//            .expect("request builder");
//        Ok(self.client.clone().request(req).await?)
        let response = format!("{{\"url to delete\": \"{}\"}}", &uri);
        let res = Response::builder()
            .body(Body::from(response))
            .unwrap();
        Ok(res)
    }

    pub async fn post(&self, payload: AclDefinition) -> Result<Response<Body>, RestError> {
        let req = Request::builder()
            .method("POST")
            .uri(&self.api)
            .body(Body::from(to_string(&payload).expect("Failed converting json to string")))
            .expect("request builder");

        Ok(self.client.clone().request(req).await?)
    }

    pub async fn get(&self, query: Option<String>) -> Result<Response<Body>, RestError> {
        let uri = format!("{}?{}", &self.api, query.unwrap_or_else(||"".to_string()));
        let req = Request::builder()
            .method("GET")
            .uri(&uri)
            .body(Body::empty())
            .expect("request builder");

        Ok(self.client.clone().request(req).await?)
    }
}

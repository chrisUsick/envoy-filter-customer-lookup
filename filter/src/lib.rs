use std::time::Duration;

use log::info;
use proxy_wasm as wasm;
use serde::Deserialize;
use wasm::traits::{Context, HttpContext};

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(wasm::types::LogLevel::Trace);
    proxy_wasm::set_http_context(
        |context_id, _root_context_id| -> Box<dyn wasm::traits::HttpContext> {
            Box::new(HelloWorld { context_id: context_id, customer_id: String::from("foo") })
        },
    )
}

struct HelloWorld {
    context_id: u32,
    customer_id: String,
}

impl wasm::traits::Context for HelloWorld {
    fn on_http_call_response(
            &mut self,
            _token_id: u32,
            _num_headers: usize,
            body_size: usize,
            _num_trailers: usize,
        ) {
        info!("http call completed");
        match self.get_http_call_response_body(0, body_size) {
            Some(body) => {
                match std::str::from_utf8(&body) {
                    Ok(v) => {
                        info!("Body: {}", v);
                        let value = self.get_customer_id(v);
                        info!("customer id: {}", value);
                        self.customer_id = value;
                    }
                    Err(e) => info!("Invalid UTF-8 sequence: {}", e),
                };
            },
            None => info!("failed to get response body")
        }
        self.resume_http_request();
        
    }
}

impl wasm::traits::HttpContext for HelloWorld {
    fn on_http_request_headers(&mut self, num_headers: usize, _end_of_stream: bool) -> wasm::types::Action {
        info!("Got {} HTTP headers in #{}.", num_headers, self.context_id);
        let headers = self.get_http_request_headers();
        let mut token = "";

        for (name, value) in &headers {
            if name == ":Authorization" {
                token = value;
            }
        }

        let res = self.dispatch_http_call(
            "customer_api", 
            vec![
                (":method", "GET"),
                (":path", "/api/lookup"),
                (":authority", "customer_api"),
                ("Authorization", token)], 
            None, 
            vec![], Duration::from_millis(500)).unwrap();
        wasm::types::Action::Pause
    }
    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> wasm::types::Action {

        self.set_http_response_header("x-customer-id", Some(&self.customer_id.as_str()));
        wasm::types::Action::Continue
    }
}

#[derive(Deserialize, Debug)]
struct CustomerLookupResponse {
    id: u32,
    name: String
}
impl HelloWorld {
    fn get_customer_id(&mut self, raw: &str) -> String {
        let r: serde_json::error::Result<CustomerLookupResponse> = serde_json::from_str(raw);
        info!("parsed json: {:#?}", r);
        r.map_or(String::from("unknown"), |res|  res.id.to_string())
    }
}

use std::time::Duration;

use log::info;
use proxy_wasm as wasm;
use wasm::traits::Context;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(wasm::types::LogLevel::Trace);
    proxy_wasm::set_http_context(
        |context_id, _root_context_id| -> Box<dyn wasm::traits::HttpContext> {
            Box::new(HelloWorld { context_id })
        },
    )
}

struct HelloWorld {
    context_id: u32,
}

impl wasm::traits::Context for HelloWorld {}

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
            vec![("Authorization", token)], 
            None, 
            vec![], Duration::from_millis(500));
        
        match res {
            Ok(_) => {
               let value = self.get_customer_id();
               self.set_http_response_header("x-customer-id", Some(value.as_str()))
            },
            Err(_) => {
                let body_string = String::from("Error, failed to get to customer api");
                let body = body_string.as_bytes();
                self.send_http_response(500, vec![], Some(body));
            }
        }

        wasm::types::Action::Continue
    }
}

impl HelloWorld {
    fn get_customer_id(&mut self) -> String {
        String::from("1")
    }
}

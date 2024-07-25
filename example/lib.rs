use envoy_dynamic_modules_rust_sdk::*;

init!(new_http_filter);

fn new_http_filter(config: &str) -> Box<dyn HttpFilter> {
    match config {
        "helloworld" => Box::new(HelloWorldFilter {}),
        "delay" => Box::new(HelloWorldFilter {}),
        "headers" => Box::new(HelloWorldFilter {}),
        "bodies" => Box::new(HelloWorldFilter {}),
        "bodies_replace" => Box::new(HelloWorldFilter {}),
        _ => panic!("Unknown config: {}", config),
    }
}

struct HelloWorldFilter {}

impl HttpFilter for HelloWorldFilter {
    fn new_instance(
        &mut self,
        _envoy_filter_instance: EnvoyFilterInstance,
    ) -> Box<dyn HttpFilterInstance> {
        Box::new(HelloWorldFilterInstance {})
    }

    fn destroy(&self) {
        println!("HelloWorldFilter destroyed");
    }
}

struct HelloWorldFilterInstance {}

impl HttpFilterInstance for HelloWorldFilterInstance {
    fn request_headers(
        &mut self,
        _request_headers: &RequestHeaders,
        _end_of_stream: bool,
    ) -> EventHttpRequestHeadersStatus {
        println!("RequestHeaders called");
        EventHttpRequestHeadersStatus::Continue
    }

    fn request_body(
        &mut self,
        _request_body: &RequestBodyBuffer,
        _end_of_stream: bool,
    ) -> EventHttpRequestBodyStatus {
        println!("RequestBody called");
        EventHttpRequestBodyStatus::Continue
    }

    fn response_headers(
        &mut self,
        _response_headers: &ResponseHeaders,
        _end_of_stream: bool,
    ) -> EventHttpResponseHeadersStatus {
        println!("ResponseHeaders called");
        EventHttpResponseHeadersStatus::Continue
    }

    fn response_body(
        &mut self,
        _response_body: &ResponseBodyBuffer,
        _end_of_stream: bool,
    ) -> EventHttpResponseBodyStatus {
        println!("ResponseBody called");
        EventHttpResponseBodyStatus::Continue
    }

    fn destroy(&mut self) {
        println!("Destroy called");
    }
}

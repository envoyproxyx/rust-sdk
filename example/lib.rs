use envoy_dynamic_modules_rust_sdk::*;

init!(new_http_filter);

fn new_http_filter(config: &str) -> Box<dyn HttpFilter> {
    match config {
        "helloworld" => Box::new(HelloWorldFilter {}),
        "delay" => Box::new(HelloWorldFilter {}), // TODO:
        "headers" => Box::new(HeadersFilter {}),
        "bodies" => Box::new(HelloWorldFilter {}), // TODO:
        "bodies_replace" => Box::new(HelloWorldFilter {}), // TODO:
        _ => panic!("Unknown config: {}", config),
    }
}

/// HelloWorldFilter is a simple filter that prints a message for each filter call.
///
/// This implements the [`HttpFilter`] trait, and will be craeted per each filter chain.
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

/// HelloWorldFilterInstance is a simple filter instance that prints a message for each filter call.
///
/// This implements the [`HttpFilterInstance`] trait, and will be created per each request.
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

/// HeadersFilter is a filter that manipulates headers.
///
/// This implements the [`HttpFilter`] trait, and will be created per each filter chain.
struct HeadersFilter {}

impl HttpFilter for HeadersFilter {
    fn new_instance(
        &mut self,
        _envoy_filter_instance: EnvoyFilterInstance,
    ) -> Box<dyn HttpFilterInstance> {
        Box::new(HeadersFilterInstance {})
    }

    fn destroy(&self) {
        println!("HeadersFilter destroyed");
    }
}

/// HeadersFilterInstance is a filter instance that manipulates headers.
///
/// This implements the [`HttpFilterInstance`] trait, and will be created per each request.
struct HeadersFilterInstance {}

impl HttpFilterInstance for HeadersFilterInstance {
    fn request_headers(
        &mut self,
        request_headers: &RequestHeaders,
        _end_of_stream: bool,
    ) -> EventHttpRequestHeadersStatus {
        if let Some(value) = request_headers.get("foo") {
            if value != "value" {
                panic!("expected this-is to be \"value\", got {:?}", value);
            } else {
                println!("foo: {}", value);
            }
        }

        request_headers
            .values("multiple-values")
            .iter()
            .for_each(|value| {
                println!("multiple-values: {}", value);
            });

        request_headers.remove("multiple-values");
        request_headers.set("foo", "yes");
        request_headers.set("multiple-values-to-be-single", "single");
        EventHttpRequestHeadersStatus::Continue
    }

    fn response_headers(
        &mut self,
        response_headers: &ResponseHeaders,
        _end_of_stream: bool,
    ) -> EventHttpResponseHeadersStatus {
        if let Some(value) = response_headers.get("this-is") {
            if value != "response-header" {
                panic!(
                    "expected this-is to be \"response-header\", got {:?}",
                    value
                );
            } else {
                println!("this-is: {}", value);
            }
        }

        response_headers
            .values("this-is-2")
            .iter()
            .for_each(|value| {
                println!("this-is-2: {}", value);
            });

        response_headers.remove("this-is-2");
        response_headers.set("this-is", "response-header");
        response_headers.set("multiple-values-res-to-be-single", "single");

        EventHttpResponseHeadersStatus::Continue
    }
}

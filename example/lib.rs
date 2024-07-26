use std::sync::{Arc, Mutex};

use envoy_dynamic_modules_rust_sdk::*;

init!(new_http_filter);

/// new_http_filter is the entry point for the filter chains.
///
/// This function is called by the Envoy corresponding to the filter chain configuration.
fn new_http_filter(config: &str) -> Box<dyn HttpFilter> {
    // Each filter is written in a way that it passes the conformance tests.
    match config {
        "helloworld" => Box::new(HelloWorldFilter {}),
        "delay" => Box::new(DelayFilter::default()),
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

/// DelayFilter is a filter that delays the request.
///
/// This implements the [`HttpFilter`] trait, and will be created per each filter chain.
struct DelayFilter {
    atomic: std::sync::atomic::AtomicUsize,
}

impl Default for DelayFilter {
    fn default() -> Self {
        DelayFilter {
            atomic: std::sync::atomic::AtomicUsize::new(1),
        }
    }
}

impl HttpFilter for DelayFilter {
    fn new_instance(
        &mut self,
        envoy_filter_instance: EnvoyFilterInstance,
    ) -> Box<dyn HttpFilterInstance> {
        let req_no = self
            .atomic
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let envoy_filter_instance = Arc::new(Mutex::new(Some(envoy_filter_instance)));
        Box::new(DelayFilterInstance {
            req_no,
            envoy_filter_instance,
        })
    }
}

/// DelayFilterInstance is a filter instance that delays the request.
///
/// This implements the [`HttpFilterInstance`] trait, and will be created per each request.
struct DelayFilterInstance {
    req_no: usize,
    envoy_filter_instance: Arc<Mutex<Option<EnvoyFilterInstance>>>,
}

impl HttpFilterInstance for DelayFilterInstance {
    fn request_headers(
        &mut self,
        _request_headers: &RequestHeaders,
        _end_of_stream: bool,
    ) -> EventHttpRequestHeadersStatus {
        if self.req_no == 1 {
            let envoy_filter_instance = self.envoy_filter_instance.clone();
            let req_no = self.req_no;
            std::thread::spawn(move || {
                println!("blocking for 1 second at RequestHeaders with id {}", req_no);
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("calling ContinueRequest with id {}", req_no);
                if let Some(envoy_filter_instance) = envoy_filter_instance.lock().unwrap().as_ref()
                {
                    envoy_filter_instance.continue_request();
                }
            });
            println!(
                "RequestHeaders returning StopAllIterationAndBuffer with id {}",
                self.req_no
            );
            EventHttpRequestHeadersStatus::StopAllIterationAndBuffer
        } else {
            println!("RequestHeaders called with id {}", self.req_no);
            EventHttpRequestHeadersStatus::Continue
        }
    }

    fn request_body(
        &mut self,
        _request_body: &RequestBodyBuffer,
        _end_of_stream: bool,
    ) -> EventHttpRequestBodyStatus {
        if self.req_no == 2 {
            let envoy_filter_instance = self.envoy_filter_instance.clone();
            let req_no = self.req_no;
            std::thread::spawn(move || {
                println!("blocking for 1 second at RequestBody with id {}", req_no);
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("calling ContinueRequest with id {}", req_no);
                if let Some(envoy_filter_instance) = envoy_filter_instance.lock().unwrap().as_ref()
                {
                    envoy_filter_instance.continue_request();
                }
            });
            println!(
                "RequestBody returning StopIterationAndBuffer with id {}",
                self.req_no
            );
            EventHttpRequestBodyStatus::StopIterationAndBuffer
        } else {
            println!("RequestBody called with id {}", self.req_no);
            EventHttpRequestBodyStatus::Continue
        }
    }

    fn response_headers(
        &mut self,
        _response_headers: &ResponseHeaders,
        _end_of_stream: bool,
    ) -> EventHttpResponseHeadersStatus {
        if self.req_no == 3 {
            let envoy_filter_instance = self.envoy_filter_instance.clone();
            let req_no = self.req_no;
            std::thread::spawn(move || {
                println!(
                    "blocking for 1 second at ResponseHeaders with id {}",
                    req_no
                );
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("calling ContinueResponse with id {}", req_no);
                if let Some(envoy_filter_instance) = envoy_filter_instance.lock().unwrap().as_ref()
                {
                    envoy_filter_instance.continue_response();
                }
            });
            println!(
                "ResponseHeaders returning StopAllIterationAndBuffer with id {}",
                self.req_no
            );

            EventHttpResponseHeadersStatus::StopAllIterationAndBuffer
        } else {
            println!("ResponseHeaders called with id {}", self.req_no);
            EventHttpResponseHeadersStatus::Continue
        }
    }

    fn response_body(
        &mut self,
        _response_body: &ResponseBodyBuffer,
        _end_of_stream: bool,
    ) -> EventHttpResponseBodyStatus {
        if self.req_no == 4 {
            let envoy_filter_instance = self.envoy_filter_instance.clone();
            let req_no = self.req_no;
            std::thread::spawn(move || {
                println!("blocking for 1 second at ResponseBody with id {}", req_no);
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("calling ContinueResponse with id {}", req_no);
                if let Some(envoy_filter_instance) = envoy_filter_instance.lock().unwrap().as_ref()
                {
                    envoy_filter_instance.continue_response();
                }
            });
            println!(
                "ResponseBody returning StopIterationAndBuffer with id {}",
                self.req_no
            );

            EventHttpResponseBodyStatus::StopIterationAndBuffer
        } else {
            println!("ResponseBody called with id {}", self.req_no);
            EventHttpResponseBodyStatus::Continue
        }
    }

    fn destroy(&mut self) {
        *self.envoy_filter_instance.lock().unwrap() = None;
    }
}

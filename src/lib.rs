#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Define the init function for the module.
/// This macro should be used in the root of the module.
/// The code block passed to the macro will be executed exactly when the module is loaded.
#[macro_export]
macro_rules! init {
    ($new_filter_fn:expr) => {
        #[no_mangle]
        pub extern "C" fn __envoy_dynamic_module_v1_event_program_init() {
            unsafe {
                NEW_HTTP_FILTER_FN = Some($new_filter_fn);
            }
        }
    };
}

static mut NEW_HTTP_FILTER_FN: Option<fn(&str) -> Box<dyn HttpFilter>> = None;

#[no_mangle]
pub extern "C" fn __envoy_dynamic_module_v1_event_http_filter_init(
    config_ptr: __envoy_dynamic_module_v1_type_HttpFilterConfigPtr,
    config_size: __envoy_dynamic_module_v1_type_HttpFilterConfigSize,
) -> __envoy_dynamic_module_v1_type_HttpFilterPtr {
    // Convert the raw pointer to the str.
    let config = unsafe {
        let slice = std::slice::from_raw_parts(config_ptr as *const u8, config_size as usize);
        std::str::from_utf8(slice).unwrap()
    };

    let boxed_filter = unsafe { NEW_HTTP_FILTER_FN.unwrap()(config) };
    let boxed_filter_ptr = Box::into_raw(boxed_filter);
    boxed_filter_ptr as *mut u8 as __envoy_dynamic_module_v1_type_HttpFilterPtr
}

#[no_mangle]
pub extern "C" fn __envoy_dynamic_module_v1_event_http_filter_destroy(
    http_filter: __envoy_dynamic_module_v1_type_HttpFilterPtr,
) {
    let http_filter = unsafe { Box::from_raw(http_filter as *mut dyn HttpFilter) };
    http_filter.destroy();
}

#[no_mangle]
pub extern "C" fn __envoy_dynamic_module_v1_event_http_filter_instance_init(
    envoy_filter_instance_ptr: __envoy_dynamic_module_v1_type_EnvoyFilterInstancePtr,
    http_filter: __envoy_dynamic_module_v1_type_HttpFilterPtr,
) -> __envoy_dynamic_module_v1_type_HttpFilterInstancePtr {
    let http_filter = unsafe { Box::from_raw(http_filter as *mut dyn HttpFilter) };

    let http_filter_instance = http_filter.new_http_filter_instance(EnvoyFilterInstance {
        raw_addr: envoy_filter_instance_ptr,
    });

    Box::into_raw(http_filter_instance) as __envoy_dynamic_module_v1_type_HttpFilterInstancePtr
}

#[no_mangle]
pub extern "C" fn __envoy_dynamic_module_v1_event_http_filter_instance_request_headers(
    http_filter_instance: __envoy_dynamic_module_v1_type_HttpFilterInstancePtr,
    request_headers_ptr: __envoy_dynamic_module_v1_type_HttpRequestHeadersMapPtr,
    end_of_stream: __envoy_dynamic_module_v1_type_EndOfStream,
) -> __envoy_dynamic_module_v1_type_EventHttpRequestHeadersStatus {
    let http_filter_instance =
        unsafe { &mut *(http_filter_instance as *mut dyn HttpFilterInstance) };
    crate::envoy_dynamic_module_v1_event_http_filter_instance_request_headers(
        http_filter_instance,
        request_headers_ptr,
        end_of_stream,
    )
}

#[no_mangle]
pub extern "C" fn __envoy_dynamic_module_v1_event_http_filter_instance_request_body(
    http_filter_instance: __envoy_dynamic_module_v1_type_HttpFilterInstancePtr,
    buffer: __envoy_dynamic_module_v1_type_HttpRequestBodyBufferPtr,
    end_of_stream: __envoy_dynamic_module_v1_type_EndOfStream,
) -> __envoy_dynamic_module_v1_type_EventHttpRequestBodyStatus {
    let http_filter_instance =
        unsafe { &mut *(http_filter_instance as *mut dyn HttpFilterInstance) };
    crate::envoy_dynamic_module_v1_event_http_filter_instance_request_body(
        http_filter_instance,
        buffer,
        end_of_stream,
    )
}

#[no_mangle]
pub extern "C" fn __envoy_dynamic_module_v1_event_http_filter_instance_response_headers(
    http_filter_instance: __envoy_dynamic_module_v1_type_HttpFilterInstancePtr,
    response_headers_map_ptr: __envoy_dynamic_module_v1_type_HttpResponseHeaderMapPtr,
    end_of_stream: __envoy_dynamic_module_v1_type_EndOfStream,
) -> __envoy_dynamic_module_v1_type_EventHttpResponseHeadersStatus {
    let http_filter_instance =
        unsafe { &mut *(http_filter_instance as *mut dyn HttpFilterInstance) };
    crate::envoy_dynamic_module_v1_event_http_filter_instance_response_headers(
        http_filter_instance,
        response_headers_map_ptr,
        end_of_stream,
    )
}

#[no_mangle]
pub extern "C" fn __envoy_dynamic_module_v1_event_http_filter_instance_response_body(
    http_filter_instance: __envoy_dynamic_module_v1_type_HttpFilterInstancePtr,
    buffer: __envoy_dynamic_module_v1_type_HttpResponseBodyBufferPtr,
    end_of_stream: __envoy_dynamic_module_v1_type_EndOfStream,
) -> __envoy_dynamic_module_v1_type_EventHttpResponseBodyStatus {
    let http_filter_instance =
        unsafe { &mut *(http_filter_instance as *mut dyn HttpFilterInstance) };
    crate::envoy_dynamic_module_v1_event_http_filter_instance_response_body(
        http_filter_instance,
        buffer,
        end_of_stream,
    )
}

#[no_mangle]
pub extern "C" fn __envoy_dynamic_module_v1_event_http_filter_instance_destroy(
    http_filter_instance: __envoy_dynamic_module_v1_type_HttpFilterInstancePtr,
) {
    let http_filter_instance =
        unsafe { Box::from_raw(http_filter_instance as *mut dyn HttpFilterInstance) };
    crate::envoy_dynamic_module_v1_event_http_filter_instance_destroy(&mut *http_filter_instance);
}

/// This function is called when request headers are received.
///
/// Made public for the usage in the generated initialization function, not for the module code.
pub fn envoy_dynamic_module_v1_event_http_filter_instance_request_headers(
    http_filter_instance: &mut dyn HttpFilterInstance,
    request_headers_ptr: __envoy_dynamic_module_v1_type_HttpRequestHeadersMapPtr,
    end_of_stream: __envoy_dynamic_module_v1_type_EndOfStream,
) -> __envoy_dynamic_module_v1_type_EventHttpRequestHeadersStatus {
    http_filter_instance
        .event_http_request_headers(
            &RequestHeaders {
                raw: request_headers_ptr,
            },
            end_of_stream == 1,
        )
        .into()
}

/// This function is called when request body data is received.
///
/// Made public for the usage in the generated initialization function, not for the module code.
pub fn envoy_dynamic_module_v1_event_http_filter_instance_request_body(
    http_filter_instance: &mut dyn HttpFilterInstance,
    buffer: __envoy_dynamic_module_v1_type_HttpRequestBodyBufferPtr,
    end_of_stream: __envoy_dynamic_module_v1_type_EndOfStream,
) -> __envoy_dynamic_module_v1_type_EventHttpRequestBodyStatus {
    http_filter_instance
        .event_http_request_body(&RequestBodyBuffer { raw: buffer }, end_of_stream == 1)
        .into()
}

/// This function is called when response headers are received.
///
/// Made public for the usage in the generated initialization function, not for the module code.
pub fn envoy_dynamic_module_v1_event_http_filter_instance_response_headers(
    http_filter_instance: &mut dyn HttpFilterInstance,
    response_headers_map_ptr: __envoy_dynamic_module_v1_type_HttpResponseHeaderMapPtr,
    end_of_stream: __envoy_dynamic_module_v1_type_EndOfStream,
) -> __envoy_dynamic_module_v1_type_EventHttpResponseHeadersStatus {
    http_filter_instance
        .event_http_response_headers(
            &ResponseHeaders {
                raw: response_headers_map_ptr,
            },
            end_of_stream == 1,
        )
        .into()
}

/// This function is called when response body data is received.
///
/// Made public for the usage in the generated initialization function, not for the module code.
pub fn envoy_dynamic_module_v1_event_http_filter_instance_response_body(
    http_filter_instance: &mut dyn HttpFilterInstance,
    buffer: __envoy_dynamic_module_v1_type_HttpResponseBodyBufferPtr,
    end_of_stream: __envoy_dynamic_module_v1_type_EndOfStream,
) -> __envoy_dynamic_module_v1_type_EventHttpResponseBodyStatus {
    http_filter_instance
        .event_http_response_body(&ResponseBodyBuffer { raw: buffer }, end_of_stream == 1)
        .into()
}

/// This function is called when the filter instance is destroyed.
///
/// Made public for the usage in the generated initialization function, not for the module code.
pub fn envoy_dynamic_module_v1_event_http_filter_instance_destroy(
    http_filter_instance: &mut dyn HttpFilterInstance,
) {
    http_filter_instance.event_http_destroy(&EnvoyFilterInstance { raw_addr: 0 });
}

/// A trait that represents a single HTTP filter in the Envoy filter chain.
/// It is used to create HttpFilterInstance(s) that correspond to each HTTP request.
///
/// This is only created once per module instance via the new_http_filter function.
pub trait HttpFilter {
    /// This is called for each new HTTP request. This should return a new HttpFilterInstance object to handle the request.
    ///
    /// Note that this must be concurrency-safe as it can be called concurrently for multiple requests.
    ///
    /// * `envoy_filter_instance` is the Envoy filter object that is used to interact with the underlying Envoy filter.
    ///   This object is unique for each HTTP request. The object is destroyed when the stream is destroyed.
    ///   Therefore, after event_http_destroy is called, the methods on this object become no-op.
    fn new_http_filter_instance(
        &mut self,
        envoy_filter_instance: EnvoyFilterInstance,
    ) -> Box<dyn HttpFilterInstance>;

    /// destroy is called when this filter is destroyed. E.g. the filter chain configuration is updated and removed from the Envoy.
    ///
    /// After this returns, the filter object is destructed.
    fn destroy(&self);
}

/// HttpFilterInstance is a trait that represents each HTTP request.
///
/// This is created for each new HTTP request and is destroyed when the request is completed.
pub trait HttpFilterInstance {
    /// This is called when request headers are received.
    /// The function should return the status of the operation.
    ///
    /// * `request_headers` is the reference to the request headers map.
    /// * `end_of_stream` is a boolean that indicates if this is the headers-only request.
    fn event_http_request_headers(
        &mut self,
        request_headers: &RequestHeaders,
        end_of_stream: bool,
    ) -> EventHttpRequestHeadersStatus;

    /// This is called when request body data is received.
    /// The function should return the status of the operation.
    ///
    /// * `request_body` is the reference to the newly arrived request body buffer.
    /// * `end_of_stream` is a boolean that indicates if this is the last data frame.
    fn event_http_request_body(
        &mut self,
        request_body: &RequestBodyBuffer,
        end_of_stream: bool,
    ) -> EventHttpRequestBodyStatus;

    /// This is called when response headers are received.
    /// The function should return the status of the operation.
    ///
    /// * `response_headers` is the reference to the response headers map.
    /// * `end_of_stream` is a boolean that indicates if this is the headers-only response.
    fn event_http_response_headers(
        &mut self,
        response_headers: &ResponseHeaders,
        end_of_stream: bool,
    ) -> EventHttpResponseHeadersStatus;

    /// This is called when response body data is received.
    /// The function should return the status of the operation.
    ///
    /// * `response_body` is the reference to the newly arrived response body buffer.
    /// * `end_of_stream` is a boolean that indicates if this is the last data frame.
    fn event_http_response_body(
        &mut self,
        response_body: &ResponseBodyBuffer,
        end_of_stream: bool,
    ) -> EventHttpResponseBodyStatus;

    /// This is called when the stream is completed or when the stream is reset.
    ///
    /// After this returns, this object is destructed.
    fn event_http_destroy(&mut self, envoy_filter_instance: &EnvoyFilterInstance);
}

/// An opaque object that represents the underlying Envoy Http filter instance.
/// This is used to interact with it from the module code.
///
/// This is a shallow wrapper around the raw pointer to the Envoy filter instance.
/// Can be copied and stored somewhere else. However, the object MUST NOT be used after the
/// [`HttpFilterInstance::event_http_destroy`] for the corresponding filter instance is called.
///
#[derive(Debug, Clone, Copy)]
pub struct EnvoyFilterInstance {
    raw_addr: __envoy_dynamic_module_v1_type_EnvoyFilterInstancePtr,
}

impl EnvoyFilterInstance {
    /// continue_request is used to resume the request processing after the filter has stopped it.
    pub fn continue_request(&self) {
        unsafe { __envoy_dynamic_module_v1_http_continue_request(self.raw_addr) }
    }

    /// continue_response is used to resume the response processing after the filter has stopped it.
    pub fn continue_response(&self) {
        unsafe { __envoy_dynamic_module_v1_http_continue_response(self.raw_addr) }
    }
}

/// An opaque object that represents the underlying Envoy Http request headers map.
/// This is used to interact with it from the module code.
///
/// This is a shallow wrapper around the raw pointer to the Envoy request headers map.
/// However, the object MUST NOT be used after the [`HttpFilterInstance::event_http_request_headers`].
pub struct RequestHeaders {
    raw: __envoy_dynamic_module_v1_type_HttpRequestHeadersMapPtr,
}

/// An opaque object that represents the underlying Envoy Http request body buffer.
/// This is used to interact with it from the module code.
pub struct RequestBodyBuffer {
    raw: __envoy_dynamic_module_v1_type_HttpRequestBodyBufferPtr,
}

/// An opaque object that represents the underlying Envoy Http response headers map.
/// This is used to interact with it from the module code.
pub struct ResponseHeaders {
    raw: __envoy_dynamic_module_v1_type_HttpResponseHeaderMapPtr,
}

/// An opaque object that represents the underlying Envoy Http response body buffer.
/// This is used to interact with it from the module code.
pub struct ResponseBodyBuffer {
    raw: __envoy_dynamic_module_v1_type_HttpResponseBodyBufferPtr,
}

/// The status of the processing after the [`HttpFilterInstance::event_http_request_headers`] is called.
pub enum EventHttpRequestHeadersStatus {
    /// Should be returned when the operation should continue.
    Continue,
    /// This indicates that Envoy shouldn't continue from processing the headers and should
    /// stop filter iteration. In other words, [`HttpFilterInstance::event_http_request_body`]
    /// will be called while not sending headers to the upstream. The header processing can be
    /// resumed by either calling [`EnvoyFilterInstance::continue_request`], or returns
    /// continue status from the [`HttpFilterInstance::event_http_request_body`].
    StopIteration,
    _reserved,
    /// This indicates that Envoy should stop all iteration and continue to buffer the request body
    /// until the limit is reached. When the limit is reached, Envoy will stop buffering and returns 500
    /// to the client. This means that [`HttpFilterInstance::event_http_request_body`] will not be called.
    ///
    /// The header processing can be resumed by either calling [`EnvoyFilterInstance::continue_request`], or
    /// returns continue status from the [`HttpFilterInstance::event_http_request_body`].
    StopAllIterationAndBuffer,
}

impl Into<__envoy_dynamic_module_v1_type_EventHttpRequestHeadersStatus>
    for EventHttpRequestHeadersStatus
{
    fn into(self) -> __envoy_dynamic_module_v1_type_EventHttpRequestHeadersStatus {
        match self {
            EventHttpRequestHeadersStatus::Continue => {
                __envoy_dynamic_module_v1_type_EventHttpRequestHeadersStatusContinue
            }
            EventHttpRequestHeadersStatus::StopIteration => {
                __envoy_dynamic_module_v1_type_EventHttpRequestHeadersStatusStopIteration
            }
            EventHttpRequestHeadersStatus::StopAllIterationAndBuffer => {
                __envoy_dynamic_module_v1_type_EventHttpRequestHeadersStatusStopAllIterationAndBuffer
            }
            _ => {
                panic!("Invalid EventHttpRequestHeadersStatus")
            }
        }
    }
}

/// The status of the processing after the [`HttpFilterInstance::event_http_response_headers`] is called.
pub enum EventHttpRequestBodyStatus {
    /// Should be returned when the operation should continue.
    Continue,
    /// This indicates that Envoy shouldn't continue from processing the body frame and should stop iteration,
    /// but continue buffering the body until the limit is reached. When the limit is reached,
    /// Envoy will stop buffering and returns 500 to the client.
    ///
    /// This stops sending body data to the upstream, so if the module wants to continue sending body
    /// data, it should call [`EnvoyFilterInstance::continue_request`] or return continue status in the
    /// subsequent [`HttpFilterInstance::event_http_request_body`] calls.
    StopIterationAndBuffer,
}

impl Into<__envoy_dynamic_module_v1_type_EventHttpRequestBodyStatus>
    for EventHttpRequestBodyStatus
{
    fn into(self) -> __envoy_dynamic_module_v1_type_EventHttpRequestBodyStatus {
        match self {
            EventHttpRequestBodyStatus::Continue => {
                __envoy_dynamic_module_v1_type_EventHttpRequestBodyStatusContinue
            }
            EventHttpRequestBodyStatus::StopIterationAndBuffer => {
                __envoy_dynamic_module_v1_type_EventHttpRequestBodyStatusStopIterationAndBuffer
            }
        }
    }
}

/// The status of the processing after the [`HttpFilterInstance::event_http_response_headers`] is called.
pub enum EventHttpResponseHeadersStatus {
    /// Should be returned when the operation should continue.
    Continue,
    /// This indicates that Envoy shouldn't continue from processing the headers and should
    /// stop filter iteration. In other words, [`HttpFilterInstance::event_http_response_body`]
    /// will be called while not sending headers to the upstream. The header processing can be
    /// resumed by either calling [`EnvoyFilterInstance::continue_response`], or returns
    /// continue status from the [`HttpFilterInstance::event_http_response_body`].
    StopIteration,
    _reserved,
    /// This indicates that Envoy should stop all iteration and continue to buffer the response body
    /// until the limit is reached. When the limit is reached, Envoy will stop buffering and returns 500
    /// to the client. This means that [`HttpFilterInstance::event_http_response_body`] will not be called.
    ///
    /// The header processing can be resumed by either calling [`EnvoyFilterInstance::continue_response`], or
    /// returns continue status from the [`HttpFilterInstance::event_http_response_body`].
    StopAllIterationAndBuffer,
}

impl Into<__envoy_dynamic_module_v1_type_EventHttpResponseHeadersStatus>
    for EventHttpResponseHeadersStatus
{
    fn into(self) -> __envoy_dynamic_module_v1_type_EventHttpResponseHeadersStatus {
        match self {
            EventHttpResponseHeadersStatus::Continue => {
                __envoy_dynamic_module_v1_type_EventHttpResponseHeadersStatusContinue
            }
            EventHttpResponseHeadersStatus::StopIteration => {
                __envoy_dynamic_module_v1_type_EventHttpResponseHeadersStatusStopIteration
            }
            EventHttpResponseHeadersStatus::StopAllIterationAndBuffer => {
                __envoy_dynamic_module_v1_type_EventHttpResponseHeadersStatusStopAllIterationAndBuffer
            }
            _ => {
                panic!("Invalid EventHttpResponseHeadersStatus")
            }
        }
    }
}

/// The status of the processing after the [`HttpFilterInstance::event_http_response_body`] is called.
pub enum EventHttpResponseBodyStatus {
    /// Should be returned when the operation should continue.
    Continue,
    /// This indicates that Envoy shouldn't continue from processing the body frame and should stop iteration,
    /// but continue buffering the body until the limit is reached. When the limit is reached,
    /// Envoy will stop buffering and returns 500 to the client.
    ///
    /// This stops sending body data to the upstream, so if the module wants to continue sending body
    /// data, it should call [`EnvoyFilterInstance::continue_responses`] or return continue status in the
    /// subsequent [`HttpFilterInstance::event_http_response_body`] calls.
    StopIterationAndBuffer,
}

impl Into<__envoy_dynamic_module_v1_type_EventHttpResponseBodyStatus>
    for EventHttpResponseBodyStatus
{
    fn into(self) -> __envoy_dynamic_module_v1_type_EventHttpResponseBodyStatus {
        match self {
            EventHttpResponseBodyStatus::Continue => {
                __envoy_dynamic_module_v1_type_EventHttpResponseBodyStatusContinue
            }
            EventHttpResponseBodyStatus::StopIterationAndBuffer => {
                __envoy_dynamic_module_v1_type_EventHttpResponseBodyStatusStopIterationAndBuffer
            }
        }
    }
}

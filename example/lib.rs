envoyx_rust_sdk::init!(new_http_filter);

fn new_http_filter(_config: &str) -> Box<dyn envoyx_rust_sdk::HttpFilter> {
    Box::new(HelloWorldFilter {})
}

struct HelloWorldFilter {}

impl envoyx_rust_sdk::HttpFilter for HelloWorldFilter {
    fn new_http_filter_instance(
        &mut self,
        _envoy_filter_instance: envoyx_rust_sdk::EnvoyFilterInstance,
    ) -> Box<dyn envoyx_rust_sdk::HttpFilterInstance> {
        todo!()
    }

    fn destroy(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_woaaaaaaaaaaarks() {
        assert_eq!(2 + 2, 4);
    }
}

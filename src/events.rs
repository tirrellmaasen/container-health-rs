use std::collections::HashMap;

type Handler = Box<dyn Fn(&str)>;

pub struct EventBus {
    handlers: HashMap<String, Vec<Handler>>,
}

impl EventBus {
    pub fn new() -> Self { Self { handlers: HashMap::new() } }
    pub fn on(&mut self, event: &str, handler: Handler) {
        self.handlers.entry(event.to_string()).or_default().push(handler);
    }
    pub fn emit(&self, event: &str, data: &str) {
        if let Some(handlers) = self.handlers.get(event) {
            for h in handlers { h(data); }
        }
    }
}

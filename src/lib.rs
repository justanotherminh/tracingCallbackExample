use tracing_subscriber::prelude::*;
use tracing_subscriber::Layer;
use std::os::raw::c_char;
use std::ffi::CString;

type LogFunc = unsafe extern "C" fn(*const c_char, u32);

struct CustomLayer {
    callback: LogFunc
}

impl CustomLayer {
    fn new(callback: LogFunc) -> Self {
        Self { callback }
    }
}

struct CustomVisitor {
    message: String
}

impl CustomVisitor {
    fn new() -> Self {
        Self { message: String::from(""), }
    }
}

impl tracing::field::Visit for CustomVisitor {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        let message = format!("  field={} value={}", field.name(), value);
        self.message.push_str(&message);
    } 
    
    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        let message = format!("  field={} value={}", field.name(), value);
        self.message.push_str(&message);
    }
    
    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        let message = format!("  field={} value={}", field.name(), value);
        self.message.push_str(&message);
    }
    
    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        let message = format!("  field={} value={}", field.name(), value);
        self.message.push_str(&message);
    }
    
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        let message = format!("  field={} value={}", field.name(), value);
        self.message.push_str(&message);
    }
    
    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        let message = format!("  field={} value={}", field.name(), value);
        self.message.push_str(&message);
    }
    
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        let message = format!("  field={} value={:?}", field.name(), value);
        self.message.push_str(&message);
    }
}

impl<S> Layer<S> for CustomLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut message = String::from("Got event!\n");
        message.push_str(&format!("  level={:?}\n", event.metadata().level()));
        message.push_str(&format!("  target={:?}\n", event.metadata().target()));
        message.push_str(&format!("  name={:?}\n", event.metadata().name()));
        let mut visitor = CustomVisitor::new();
        event.record(&mut visitor);
        message.push_str(&visitor.message);
        let c_string = CString::new(message).expect("Failed to convert to CString");
        unsafe {
            (self.callback)(c_string.as_ptr() as *const c_char, c_string.to_bytes().len() as u32);
        }
    }
}

#[no_mangle]
pub extern "C" fn run_callback(callback: LogFunc) {
    let my_layer = CustomLayer::new(callback);
    // Set up how `tracing-subscriber` will deal with tracing data.
    tracing_subscriber::registry().with(my_layer).init();

    // Log something simple. In `tracing` parlance, this creates an "event".
    tracing::info!(a_bool = true, answer = 42, message = "Hello World from Rust!");
    tracing::error!("This is an error");
}
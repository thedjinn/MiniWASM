//! This is a minimal WebAssembly application. It was designed to showcase what must be done to get
//! Rust-powered WebAssembly going with only a small amount of code, while still providing useful
//! tools, such as integration with console.log.

use std::cell::RefCell;
use std::panic;

/// Use wee_alloc as the global allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

thread_local! {
    /// Assign some memory to global state. We don't use lazy_alloc or something similar to prevent
    /// pulling in a lot of threading code. This of course limits this example to a single threaded
    /// application, but that's fine for most use cases.
    /// 
    /// Note: wrapping the RefCell in a Box appears to result in smaller code size
    static APP: Box<RefCell<App>> = Box::new(RefCell::new(App::new()));
}

/// A custom panic handler that delegates to console.error so we can see panics inside the browser
/// console.
fn panic_handler(info: &panic::PanicInfo) {
    error(&info.to_string());
}

/// Import some callbacks that map to console functions.
extern "C" {
    #[link_name="console_log"]
    fn _console_log(a_ptr: *const u8, a_len: usize);

    #[link_name="console_error"]
    fn _console_error(a_ptr: *const u8, a_len: usize);
}

/// A helper function to wrap an unsafe external callback (that requires memory addresses) with a
/// function that accepts a &str. The &str is split into an address/length tuple so the
/// JavaScript-side knows where to find its argument.
fn wrap(s: &str, f: unsafe extern "C" fn(*const u8, usize)) {
    let ptr = s.as_ptr();
    let len = s.len();

    unsafe {
        f(ptr, len);
    }
}

/// Forward the provided &str to JavaScript's console.log
pub fn log(s: &str) {
    wrap(s, _console_log);
}

/// Forward the provided &str to JavaScript's console.error
pub fn error(s: &str) {
    wrap(s, _console_error);
}

/// Initialization entry point for the WebAssembly module. This sets a new panic handler and calls
/// the initialize function on the global App instance.
#[no_mangle]
pub extern "C" fn initialize() {
    panic::set_hook(Box::new(panic_handler));

    APP.with(|k| k.borrow_mut().initialize());
}

/// Example exported custom function. This one calls a method on the App instance and returns its
/// result. Note that calling methods from the App instance is optional, but this approach makes
/// things more flexible by providing a container for global state.
#[no_mangle]
pub extern "C" fn hello(arg: u32) -> u32 {
    APP.with(|k| k.borrow_mut().hello(arg))
}

/// The WebAssembly application's global state struct.
struct App {
}

impl App {
    /// Create a new App instance. This is called by thread_local! on first use of the global
    /// instance.
    fn new() -> Self {
        Self {
        }
    }

    /// Initialize the application here.
    fn initialize(&self) {
        log("App initialized");
    }

    /// Example custom function
    fn hello(&self, arg: u32) -> u32 {
        log("Hello, world!");

        // Return an example value. Note that WebAssembly's calling convention only allows us to
        // accept/return primitive types and pointers to said types.
        arg * arg
    }
}

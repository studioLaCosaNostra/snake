#[macro_use]
extern crate stdweb;

fn your_function_to_export() -> u32 {
    return 4
}

fn main() {
    stdweb::initialize();
    stdweb::web::alert("hello");
    js! {
        window.your_library = {};
        window.your_library.your_function = @{your_function_to_export};
    }
    stdweb::event_loop();
}

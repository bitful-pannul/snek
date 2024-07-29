use kinode_process_lib::{await_message, call_init, http, println, Address};

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
});

call_init!(init);
fn init(our: Address) {
    println!("hello");

    http::serve_index_html(&our, "ui", false, false, vec!["/"]).unwrap();

    loop {
        match await_message() {
            _ => {}
        };
    }
}

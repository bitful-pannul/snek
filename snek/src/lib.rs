use kinode_process_lib::{await_message, call_init, homepage, http, println, Address};
static SNEK_SVG: &str = include_str!("../../pkg/ui/snek.svg");

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v1",
});

call_init!(init);
fn init(our: Address) {
    println!("-ssssss-");
    let mut server = http::server::HttpServer::new(5);
    server.serve_ui(&our, "/ui", vec!["/"], http::server::HttpBindingConfig::default()).unwrap();
    homepage::add_to_homepage("snek", Some(SNEK_SVG), Some("/"), None);
    loop {
        match await_message() {
            _ => {}
        };
    }
}

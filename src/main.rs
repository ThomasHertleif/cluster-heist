extern crate actix_web;
extern crate crossbeam_deque;
extern crate notify;

use actix_web::{server, App, HttpRequest, HttpResponse};
use std::thread;

fn index(req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    println!("hello");

    let d = Deque::new();
    let s = d.stealer();

    thread::spawn(move || {
        server::new(
            || App::new()
                .resource("/", |r| r.f(index)))
            .bind("127.0.0.1:8088").expect("Can not bind to 127.0.0.1:8088")
            .run();
    });


    use notify::{watcher, DebouncedEvent}
    use std::sync::mpsc::channel;
    use std::time::Duration;

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("./test/files", RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
           Ok(DebouncedEvent::Create(path)) => d.push(path),
           Ok(whatever) => println!("wayne: {:?}", whatever),
           Err(e) => println!("watch error: {:?}", e),
        }
    }
}
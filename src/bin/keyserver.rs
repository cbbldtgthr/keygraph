use rdev::{listen, EventType};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};
use tungstenite::accept;

pub fn main() {
    let (tx, rx) = std::sync::mpsc::channel();

    let websocket_thread = thread::spawn(move || {
        let rx = Arc::new(Mutex::new(rx));
        run_websocket_server(rx).unwrap();
    });

    if let Err(error) = listen(move |event| {
        if let EventType::KeyPress(key) = event.event_type {
            // let k = event.name.map(|l| l as u16).clone();
            let msg = KeyMsg {
                key: format!("{:?}", key),
                name: event.clone().name.unwrap(),
            };
            if tx.send(serde_json::to_string(&msg).unwrap()).is_err() {
                println!("Failed to send key event");
            };
        }
    }) {
        eprintln!("Error: {:?}", error);
    }

    websocket_thread.join().unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct KeyMsg {
    pub key: String,
    pub name: String,
}

fn run_websocket_server(
    rx: Arc<Mutex<std::sync::mpsc::Receiver<String>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let server = TcpListener::bind("127.0.0.1:3030")?;
    println!("WebSocket server running on ws://127.0.0.1:3030");

    for stream in server.incoming() {
        let rx = Arc::clone(&rx);

        thread::spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {
                let rx_lock = rx.lock().unwrap();
                let now = Instant::now();
                if let Ok(mut key_event) = rx_lock.recv() {
                    thread::sleep(Duration::from_millis(10));
                    if let Ok(key_event_debounce) = rx_lock.try_recv() {
                        // Discard previous event if next event is within 10ms
                        // This solves the MacOs "combo key" issuE
                        key_event = key_event_debounce;
                    }
                    let message = key_event;

                    println!("{:?},{:?}", message, now.elapsed().as_nanos());
                    if websocket.send(tungstenite::Message::Text(message)).is_err() {
                        println!("Client disconnected");
                        break;
                    }
                }
            }
        });
    }
    Ok(())
}

use std::thread;

use rdev::{listen, EventType};

// use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;
// use iced::futures::stream::StreamExt;
use iced::futures::Stream;
use iced::stream;
use iced::widget::{column, text};
use iced::{Element, Subscription, Task};
// use std::sync::mpsc
use tokio::sync::mpsc;

struct KeyGraph {
    last_key: String,
}

#[derive(Debug, Clone)]
enum Message {
    KeyPressed(String),
}

impl KeyGraph {
    fn new() -> (KeyGraph, Task<Message>) {
        (
            KeyGraph {
                last_key: "".to_string(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::KeyPressed(key) => {
                self.last_key = key;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text("Last key pressed:").size(24),
            text(&self.last_key).size(48)
        ]
        .into()
    }
}

fn some_worker() -> impl Stream<Item = String> {
    stream::channel(100, |mut output| async move {
        let (tx, mut rx) = mpsc::channel(100);

        thread::spawn(move || {
            println!("{:?}", "0");
            listen(move |event| {
                println!("{:?}", "0.1");
                if let EventType::KeyPress(key) = event.event_type {
                    let _ = tx.send(format!("{:?}", key));
                }
            })
            .unwrap();
        });

        loop {
            println!("{:?}", "1");
            let input = rx.recv().await;
            println!("{:?}", "2");
            let _ = output.send(input.unwrap()).await;
            println!("{:?}", "3");
        }
    })
}

pub fn main() -> iced::Result {
    iced::application("KeyGraph", KeyGraph::update, KeyGraph::view).run_with(KeyGraph::new)
}

use std::thread;

use rdev::{listen, EventType};

use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;
use iced::futures::stream::StreamExt;
use iced::futures::Stream;
use iced::stream;
use iced::widget::{column, text};
use iced::{Element, Subscription, Task};

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
        let (mut tx, mut rx) = mpsc::channel(100);

        thread::spawn(move || {
            listen(move |event| {
                if let EventType::KeyPress(key) = event.event_type {
                    let _ = tx.send(format!("{:?}", key));
                }
            })
            .unwrap();
        });

        loop {
            let input = rx.select_next_some().await;
            let _ = output.send(input).await;
        }
    })
}

fn subscription(_state: &KeyGraph) -> Subscription<Message> {
    Subscription::run(some_worker).map(|key_str| Message::KeyPressed(key_str))
}

pub fn main() -> iced::Result {
    iced::application("KeyGraph", KeyGraph::update, KeyGraph::view)
        .subscription(subscription)
        .run_with(KeyGraph::new)
}

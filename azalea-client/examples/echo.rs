//! A simple bot that repeats chat messages sent by other players.

use azalea_client::{Account, Client, Event};

#[tokio::main]
async fn main() {
    env_logger::init();
    // deadlock detection, you can safely delete this block if you're not trying to
    // debug deadlocks in azalea
    {
        use parking_lot::deadlock;
        use std::thread;
        use std::time::Duration;
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(10));
            let deadlocks = deadlock::check_deadlock();
            if deadlocks.is_empty() {
                continue;
            }
            println!("{} deadlocks detected", deadlocks.len());
            for (i, threads) in deadlocks.iter().enumerate() {
                println!("Deadlock #{i}");
                for t in threads {
                    println!("Thread Id {:#?}", t.thread_id());
                    println!("{:#?}", t.backtrace());
                }
            }
        });
    }

    let account = Account::offline("bot");
    // or let account = Account::microsoft("email").await;

    let (client, mut rx) = Client::join(&account, "localhost").await.unwrap();

    while let Some(event) = rx.recv().await {
        match &event {
            Event::Chat(m) => {
                if let (Some(sender), content) = m.split_sender_and_content() {
                    if sender == client.profile.name {
                        continue; // ignore our own messages
                    }
                    client.chat(&content);
                };
            }
            _ => {}
        }
    }
}

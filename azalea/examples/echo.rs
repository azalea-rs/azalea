use azalea::{Account, Event};

let account = Account::offline("bot");
// or let account = azalea::Account::microsoft("access token").await;

let bot = account.join("localhost".try_into().unwrap()).await.unwrap();

loop {
    match bot.next().await {
        Event::Message(m) {
            if m.username == bot.username { return };
            bot.chat(m.message).await;
        },
        Event::Kicked(m) {
            println!(m);
            bot.reconnect().await.unwrap();
        },
        Event::Hunger(h) {
            if !h.using_held_item() && h.hunger <= 17 {
                match bot.hold(azalea::ItemGroup::Food).await {
                    Ok(_) => {},
                    Err(e) => {
                        println!("{}", e);
                        break;
                    }
                }
                match bot.use_held_item().await {
                    Ok(_) => {},
                    Err(e) => {
                        println!("{}", e);
                        break;
                    }
                }
            }
        }
        _ => {}
    }
}

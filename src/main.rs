#![feature(custom_derive)]
extern crate telegram_bot;
extern crate chrono;

mod dates;

use telegram_bot::*;

fn main() {
    let api = Api::from_token("125119585:AAFmq_oBJQdlkqPqSXZ2Ml7qMVv_paa0mQc").unwrap();
    println!("getMe: {:?}", api.get_me());
    let mut listener = api.listener(ListeningMethod::LongPoll(None));

    let res = listener.listen(|u| {
       if let Some(m) = u.message {
           let name = m.from.first_name;

           match m.msg {
               MessageType::Text(t) => {
                   println!("<{}> {}", name, t);

                   if t == "/exit" {
                       return Ok(ListeningAction::Stop);
                   }

                   if name == "Tom" {
                       try!(api.send_message(m.chat.id(),
                                        format!("Go away, Beardo."),
                                        None, None, None, None));
                   } else {
                       let di = dates::DateIterator::new(|&x| x.checked_add(chrono::Duration::days(1)));
                       let ds: Vec<_> = di.take(28).collect();

                       let mut kb: Vec<Vec<String>> = Vec::new();
                       for (i, d) in ds.into_iter().enumerate() {
                           let row = i / 7;
                           if kb.len() < row+1 {
                               kb.push(Vec::new());
                           }
                           kb[row].push(d.format("%d %b").to_string());
                       }

                       let keyboard = ReplyKeyboardMarkup {
                           keyboard: kb,
                            one_time_keyboard: Some(true),
                            .. Default::default()
                       };
                     try!(api.send_message(m.chat.id(),
                                          format!("Hi, {}!",
                                                 name),
                                          None, None, None, Some(keyboard.into())));
                   }

               },
               _ => {}
           }
       }

        Ok(ListeningAction::Continue)
    });

    if let Err(e) = res {
        println!("An error occured: {}", e);
    }
}

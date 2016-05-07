#![feature(custom_derive)]
extern crate telegram_bot;
extern crate chrono;

use telegram_bot::*;
use std::ops::*;

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
                       let keyboard = ReplyKeyboardMarkup {
                           keyboard: vec![vec!["1".into(), "2".into(), "3".into(), "4".into()],
                                     vec!["Yes".into(), "No".into()]],
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

#[derive(Display, Debug)]
struct DateCollector {
    curr: chrono::NaiveDate,
    next: chrono::NaiveDate,
}

impl Iterator for DateCollector {
    type Item = chrono::NaiveDate;

    fn next(&mut self) -> Option<chrono::NaiveDate> {
        let new_next = self.curr.add(chrono::Duration::days(1));

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

#[test]
fn it_works()
{
    let test_date = chrono::Local::today().naive_local();
    let mut d = DateCollector { curr: test_date, next: test_date };
    assert_eq!(chrono::Local::today().naive_local(), d.next().unwrap());
    assert_eq!(chrono::Local::today().naive_local().add(chrono::Duration::days(1)), d.next().unwrap());
    println!("{:?}", d)
}

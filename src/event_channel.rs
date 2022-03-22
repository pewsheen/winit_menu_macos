use crate::event::Event;
use once_cell::sync::OnceCell;
use std::sync::{
  mpsc::{channel, Receiver, Sender},
  Mutex,
};

pub fn get_event_channel() -> &'static (Mutex<Sender<Event>>, Mutex<Receiver<Event>>) {
  static CHANNEL: OnceCell<(Mutex<Sender<Event>>, Mutex<Receiver<Event>>)> = OnceCell::new();
  CHANNEL.get_or_init(|| {
    let (tx, rx) = channel::<Event>();
    (Mutex::new(tx), Mutex::new(rx))
  })
}

use crate::event::Event;
use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::OnceCell;

/// Event channel for receiving events from the menu click.
pub fn get_event_channel() -> &'static (Sender<Event>, Receiver<Event>) {
  static CHANNEL: OnceCell<(Sender<Event>, Receiver<Event>)> = OnceCell::new();
  CHANNEL.get_or_init(|| unbounded::<Event>())
}

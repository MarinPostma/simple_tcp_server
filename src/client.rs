use crate::channel::Channel;
use std::thread::JoinHandle;

pub type Uid = usize;

pub struct Client<T> {
    pub id: Uid,
    pub nick: String,
    pub channel: Channel<T>,
    pub thread: JoinHandle<()>,
    pub open: bool,
}

impl<T> Client<T> {
    pub fn new(id: Uid, channel: Channel<T>, thread: JoinHandle<()>) -> Self {
        Self {
            id: id,
            nick: format!("user{}", id),
            channel: channel,
            thread: thread,
            open: true,
        }
    }

    pub fn set_nick(&mut self, name: String) {
        self.nick = name;
    }
    pub fn close(&mut self) {
        self.open = false;
    }
}

use std::sync::mpsc::*;

pub struct Channel<T>(Sender<T>, Receiver<T>);

impl<T> Channel<T> {
    pub fn new() -> (Self, Self) {
        let (c_tx, c_rx) = channel();
        let (s_tx, s_rx) = channel();
        (Self(c_tx, s_rx), Self(s_tx, c_rx))
    }

    pub fn send(&self, t: T) -> Result<(), SendError<T>> {
        self.0.send(t)
    }

    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.1.try_recv()
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        self.1.recv()
    }

    pub fn iter(&self) -> Iter<T> {
        self.1.iter()
    }
}

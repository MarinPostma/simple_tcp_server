pub mod channel;
pub mod client;
pub mod message;

use channel::Channel;
use client::{Client, Uid};
use message::Message;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::thread;

enum Action {
    Disconnect,
    Message(Message),
}

fn greet_client(writer: &mut BufWriter<TcpStream>) {
    writer
        .write("\nwelcome to the super server\n".as_bytes())
        .unwrap();
    writer.flush().unwrap();
}

fn handle_client(uid: Uid, server: Channel<Action>, stream: TcpStream) {
    let mut writer = BufWriter::new(stream.try_clone().unwrap());
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut buf = String::new();

    stream.set_nonblocking(true).unwrap();
    greet_client(&mut writer);
    loop {
        // (1) check for message to send to server for broadcasting
        match server.try_recv() {
            Ok(Action::Message(message)) => {
                writer
                    .write(
                        format!(
                            "{}: {}",
                            // the sender message will be labelled "you"
                            if message.sender_id == uid {
                                "you".into()
                            } else {
                                message.sender_name.unwrap()
                            },
                            message.content
                        )
                        .as_bytes(),
                    )
                    .unwrap();
                writer.flush().unwrap();
            }
            Ok(_) => {}
            Err(_) => {}
        }
        // (2) check for new messages to send to the client
        match reader.read_line(&mut buf) {
            Ok(0) => {
                server.send(Action::Disconnect).unwrap();
                return;
            }
            Ok(_) => {
                println!("message received: {}", buf);
                let message = Message::new(uid, &buf.clone());
                server.send(Action::Message(message)).unwrap();
                buf.clear();
            }
            Err(err) => { }
        }
    }
}

fn main() -> io::Result<()> {
    let mut next_id = 0;
    let listener = TcpListener::bind("127.0.0.1:5555")?;

    //set tcp listener in nonblocking mode so the accept call don't hang main thread.
    listener.set_nonblocking(true)?;

    let mut clients = HashMap::new();

    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                println!("A new client connected");

                let (client, server) = Channel::new();
                let thread = thread::spawn(move || {
                    handle_client(next_id, server, stream);
                    println!("client {} has disconnected", next_id);
                });
                // add new client
                clients.insert(next_id, Client::new(next_id, client, thread));
                next_id += 1;
            }
            Err(_) => {
                //log error eventually
            }
        }

        // check for disconected clients
        let mut messages = vec![];
        for (_, client) in &mut clients.iter_mut() {
            // (1) read to check for new message
            if let Ok(action) = client.channel.try_recv() {
                match action {
                    Action::Disconnect => {
                        // remove client from dic
                        client.close();
                    }
                    Action::Message(mut message) => {
                        message.sender_name = Some(client.nick.clone());
                        messages.push(message);
                    }
                }
            }
        }

        // broadcast received messages
        clients.values().filter(|c| c.open).for_each(|c| {
            messages.iter().for_each(|m| {
                c.channel.send(Action::Message(m.clone())).unwrap();
            });
        });

        // remove disconnected clients
        clients = clients.into_iter().filter(|(_, v)| v.open).collect();
    }
}

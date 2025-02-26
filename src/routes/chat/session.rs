// ClientSession is an actor, it manages peer tcp connection
// and proxies commands from peer to ChatServer.
use std::{
    io, net,
    str::FromStr,
    time::{Duration, Instant},
};  
use std::borrow::BorrowMut;     // to use as_mut()

// spawn : spawns a future on the current thread as a new task
// spawning allows the process of running a new asynchronous task in the background
// this allows us to continue executing other code while it runs
use actix::{prelude::*, spawn};
use tokio::{
    io::{split, WriteHalf},
    net::{TcpListener, TcpStream},
};
use tokio_util::codec::FramedRead;

// use crate::routes::chat::codec::{ChatCodec, ChatRequest, ChatResponse};
use crate::routes::chat::codec::ChatCodec;
use crate::routes::chat::codec::ChatRequest;
use crate::routes::chat::codec::ChatResponse;
use crate::routes::chat::websocket::{self, ChatServer};

// to inherit Serialize and Deserialize traits for structs
use serde::{Serialize, Deserialize};        
use serde::*;

// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

// `ChatSession` actor is responsible for tcp peer communications.
pub struct ChatSession {
    /// unique session id
    id: usize,
    /// this is address of chat server
    addr: Addr<ChatServer>,
    /// Client must send ping at least once per 10 seconds, otherwise we drop
    /// connection.
    hb: Instant,
    /// joined room
    room: String,
    /// Framed wrapper
    framed: actix::io::FramedWrite<ChatResponse, WriteHalf<TcpStream>, ChatCodec>,
}

impl Actor for ChatSession {
    // For tcp communication we are going to use `FramedContext`.
    // It is convenient wrapper around `Framed` object from `tokio_io`
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        let addr = ctx.address();
        self.addr
            .send(websocket::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                actix::fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(websocket::Disconnect { id: self.id });
        Running::Stop
    }
}

impl actix::io::WriteHandler<io::Error> for ChatSession {}

/// To use `Framed` we have to define Io type and Codec
impl StreamHandler<Result<ChatRequest, io::Error>> for ChatSession {
    /// This is main event loop for client requests
    fn handle(&mut self, msg: Result<ChatRequest, io::Error>, ctx: &mut Context<Self>) {
        match msg {
            Ok(ChatRequest::List) => {
                // Send ListRooms message to chat server and wait for response
                println!("List rooms");
                self.addr
                    .send(websocket::ListRooms)
                    .into_actor(self)
                    .then(|res, act, _| {
                        match res {
                            Ok(rooms) => {
                                act.framed.write(ChatResponse::Rooms(rooms));
                            }
                            _ => println!("Something is wrong"),
                        }
                        actix::fut::ready(())
                    })
                    .wait(ctx)
                // .wait(ctx) pauses all events in context,
                // so actor wont receive any new messages until it get list of rooms back
            }
            Ok(ChatRequest::Join(name)) => {
                println!("Join to room: {name}");
                name.clone_into(&mut self.room);
                self.addr.do_send(websocket::Join {
                    id: self.id,
                    name: name.clone(),
                });
                self.framed.write(ChatResponse::Joined(name));
            }
            Ok(ChatRequest::Message(message)) => {
                // send message to chat server
                println!("Peer message: {message}");
                let constructed_websocket_message = websocket::Message {
                    id: self.id,
                    msg: message.clone(),
                    room: self.room.clone(),
                };

                let websocket_message_copy = websocket::Message {
                    id: self.id,
                    msg: message.clone(),
                    room: self.room.clone(),
                };
                // let websocket_message_copy = constructed_websocket_message;
                // let websocket_message_copy_2 = websocket_message_copy;
                self.addr.do_send(websocket_message_copy);
                

                // self.addr.do_send(websocket::Message {
                //     id: self.id,
                //     msg: message,
                //     room: self.room.clone(),
                // });
                println!("Message printout : {:?}", constructed_websocket_message);
            }
            // we update heartbeat time on ping from peer
            Ok(ChatRequest::Ping) => self.hb = Instant::now(),
            _ => ctx.stop(),
        }
    }
}

/// Handler for Message, chat server sends this message, we just send string to
/// peer
impl Handler<Message> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: Message, _: &mut Context<Self>) {
        // send message to peer
        self.framed.write(ChatResponse::Message(msg.0));
    }
}

/// Helper methods
impl ChatSession {
    pub fn new(
        addr: Addr<ChatServer>,
        framed: actix::io::FramedWrite<ChatResponse, WriteHalf<TcpStream>, ChatCodec>,
    ) -> ChatSession {
        ChatSession {
            id: 0,
            addr,
            hb: Instant::now(),
            room: "main".to_owned(),
            framed,
        }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method check heartbeats from client
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_interval(Duration::new(1, 0), |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > Duration::new(10, 0) {
                // heartbeat timed out
                println!("Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(websocket::Disconnect { id: act.id });

                // stop actor
                ctx.stop();
            }

            act.framed.write(ChatResponse::Ping);
            // if we can not send message to sink, sink is closed (disconnected)
        });
    }
}

// Define TCP server that will accept incoming TCP connection and create
// chat actors.
pub fn tcp_server(_s: &str, server: Addr<ChatServer>) {
    // Create server listener
    let addr = net::SocketAddr::from_str("127.0.0.1:12345").unwrap();

    spawn(async move {
        let listener = TcpListener::bind(&addr).await.unwrap();

        while let Ok((stream, _)) = listener.accept().await {
            let server = server.clone();
            ChatSession::create(|ctx| {
                let (r, w) = split(stream);
                ChatSession::add_stream(FramedRead::new(r, ChatCodec), ctx);
                ChatSession::new(server, actix::io::FramedWrite::new(w, ChatCodec, ctx))
            });
        }
    });
}

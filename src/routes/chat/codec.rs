#![allow(dead_code)]
use std::io;
use actix::prelude::*;

// Decoder and Encoder are both traits
// this to encrypt and decrypt messages within websocket stream
use actix_codec::{Decoder, Encoder};
use actix_web::web::{BufMut, BytesMut};

// BigEndian : A big-endian system stores the most significant byte of a word in the smallest possible memory address
//
// Big Endian (ctd.) : stores the least signficant byte in the largest possible memory address.
// ByteOrder : trait that describes types that can serialize integers as bytes.
use byteorder::{BigEndian, ByteOrder};
use serde::{Deserialize, Serialize};
use serde_json as json; 

// Client request
#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
#[serde(tag = "cmd", content = "data")]
pub enum ChatRequest {
    // List rooms
    List,
    // Join rooms
    Join(String),
    // Send message
    Message(String),
    // Ping
    Ping,
}

// Server response
#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
#[serde(tag = "cmd", content = "data")]
pub enum ChatResponse {
    Ping,

    // List of rooms
    Rooms(Vec<String>),

    // Joined
    Joined(String),

    // Message
    Message(String),
}

// Codec for Client -> Server transport
pub struct ChatCodec;

// decode is a method that is part of the Decoder trait that is being overridden
impl Decoder for ChatCodec {
    type Item = ChatRequest;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let size = {
            if src.len() < 2 {
                return Ok(None);
            }
            BigEndian::read_u16(src.as_ref()) as usize
        };

        if src.len() >= size + 2 {
            let _ = src.split_to(2);
            let buf = src.split_to(size);
            Ok(Some(json::from_slice::<ChatRequest>(&buf)?))
        } else {
            Ok(None)
        }
    }
}

// Encoder<ChatResponse> : ChatResponse is a type to be inherited as a parameter for msg
// encode is a method part of the Encoder trait that is being overriden
impl Encoder<ChatResponse> for ChatCodec {
    type Error = io::Error;

    fn encode(&mut self, msg: ChatResponse, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let msg = json::to_string(&msg).unwrap();
        let msg_ref: &[u8] = msg.as_ref();

        dst.reserve(msg_ref.len() + 2);
        dst.put_u16(msg_ref.len() as u16);
        dst.put(msg_ref);

        Ok(())
    }
}

// Codec for Server -> Client transport
// struct with no predefined field based values
pub struct ClientChatCodec;

// implements decode trait for ClientChatCodec
impl Decoder for ClientChatCodec {
    type Item = ChatResponse;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let size = {
            if src.len() < 2 {
                return Ok(None);
            }
            BigEndian::read_u16(src.as_ref()) as usize
        };

        if src.len() >= size + 2 {
            let _ = src.split_to(2);
            let buf = src.split_to(size);
            Ok(Some(json::from_slice::<ChatResponse>(&buf)?))
        } else {
            Ok(None)
        }
    }
}

impl Encoder<ChatRequest> for ClientChatCodec {
    type Error = io::Error;

    fn encode(&mut self, msg: ChatRequest, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let msg = json::to_string(&msg).unwrap();
        let msg_ref: &[u8] = msg.as_ref();

        dst.reserve(msg_ref.len() + 2);
        dst.put_u16(msg_ref.len() as u16);
        dst.put(msg_ref);

        Ok(())
    }
}

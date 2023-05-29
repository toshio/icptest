#![allow(non_snake_case)]

use ic_cdk::{
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    },
    query, update,
};
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Clone, Debug, CandidType, Deserialize)]
enum Content {
    Text(String),
    Image(Vec<u8>),
    Video(Vec<u8>),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Message {
    content: Content,
    vote: i128,
    creator: Principal
}

thread_local! {
    static MESSAGE_ID: RefCell<u128> = RefCell::new(0);
    static WALL: RefCell<BTreeMap<u128, Message>> = RefCell::new(BTreeMap::new());
}

#[update]
fn writeMessage(content: Content) -> u128 {
    MESSAGE_ID.with(|message_id| {
        let message = Message {
            creator: ic_cdk::api::caller(),
            vote: 0,
            content
        };

        let mut id = message_id.borrow_mut();
        WALL.with(|wall| {
            wall.borrow_mut().insert(*id, message);
        });
        *id += 1;
        *id - 1
    })
}

#[query]
fn getMessage(messageId: u128) -> Result<Message, String> {
    WALL.with(|wall| {
        match wall.borrow().get(&messageId) {
            Some(message) => Ok(message.clone()),
            None => Err(format!("Message not found: {}", messageId))
        }
    })
}

#[update]
fn updateMessage(messageId: u128, content: Content) -> Result<(), String> {
    WALL.with(|wall| {
        let mut w = wall.borrow_mut();
        match w.get_mut(&messageId) {
            Some(message) => {
                let caller = &ic_cdk::api::caller();
                if message.creator.eq(&caller) {
                    message.content = content;
                    Ok(())
                } else {
                    Err(String::from("Not owner"))
                }
            },
            None => {
                Err(format!("Message not found: {}", messageId))
            }
        }
    })
}

#[update]
fn deleteMessage(messageId : u128) -> Result<(), String> {
    WALL.with(|wall| {
        match wall.borrow_mut().remove(&messageId) {
            Some(_) => Ok(()),
            None => Err(format!("Message not found: {}", messageId))
        }
    })
}

#[update]
fn upVote(messageId: u128) -> Result<(), String> {
    WALL.with(|wall| {
        let mut w = wall.borrow_mut();
        match w.get_mut(&messageId) {
            Some(message) => {
                message.vote += 1;
                Ok(())
            },
            None => {
                Err(format!("Message not found: {}", messageId))
            }
        }
    })
}

#[update]
fn downVote(messageId: u128) -> Result<(), String> {
    WALL.with(|wall| {
        let mut w = wall.borrow_mut();
        match w.get_mut(&messageId) {
            Some(message) => {
                message.vote -= 1;
                Ok(())
            },
            None => {
                Err(format!("Message not found: {}", messageId))
            }
        }
    })
}

#[query]
fn getAllMessages() -> Vec<Message> {
    WALL.with(|wall| {
        wall.borrow().values().cloned().collect()
    })
}

#[query]
fn getAllMessagesRanked() -> Vec<Message> {
    WALL.with(|wall| {
        let mut values: Vec<Message> = wall.borrow().values().cloned().collect();
        // values.sort_by(|a, b| b.vote.cmp(&a.vote)); // Decending order
        values.sort_by(|a, b|
            if a.vote == b.vote {
                std::cmp::Ordering::Equal
            } else if a.vote > b.vote {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            });
        values
    })
}

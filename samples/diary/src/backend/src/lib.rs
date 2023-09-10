mod user;

use user::{User, Diary};
use candid::{Principal, CandidType, Deserialize};
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::{
  post_upgrade, pre_upgrade, query, storage,
  update,
};

type Date = u32; // YYYYYMMDD

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct UserConfig {
  public: bool,
  name: String,
  title: String,
  description: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Header {
  date: Date,
  title: String,
}

thread_local! {
  static USERS: RefCell<HashMap<Principal, User>> = RefCell::new(HashMap::new());
}

#[update]
pub fn set(config:UserConfig) -> Result<UserConfig, String> {
  let principal: Principal = ic_cdk::caller();
  USERS.with(|users| {
    let mut users = users.borrow_mut();
    match users.get_mut(&principal) {
      Some(user) => {
        user.update(
          config.public,
          config.name,
          config.title,
          config.description,
        );
      },
      None => {
        let user = User::new(
          principal,
          config.public,
          config.name,
          config.title,
          config.description,
        );
        users.insert(principal, user);
      }
    };
  });
  get(principal)
}

#[query]
pub fn get(principal:Principal) -> Result<UserConfig, String> {
  USERS.with(|users| {
    let users = users.borrow();
    match users.get(&principal) {
      Some(user) => {
        if user.allowed(ic_cdk::caller()) {
          let config = UserConfig {
            public:      user.public,
            name:        user.name.clone(),
            title:       user.title.clone(),
            description: user.description.clone(),
          };
          Ok(config)
        } else {
          Err(String::from("Private"))
        }
      },
      None => Err(String::from("No user"))
    }
  })
}

#[update]
pub fn save(diary:Diary) -> Result<Diary, String> {
  let principal: Principal = ic_cdk::caller();
  USERS.with(|users| {
    let mut users = users.borrow_mut();
    let user = users.entry(principal).or_insert(
      User::new(
        principal,
        false,
        String::from(""),
        String::from(""),
        String::from("")
      )
    );
    user.save(diary)
  })
}

#[update]
pub fn delete(date:Date) -> Result<Diary, String> {
  let principal: Principal = ic_cdk::caller();
  USERS.with(|users| {
    let mut users = users.borrow_mut();
    match users.get_mut(&principal) {
      Some(user) => {
        match user.delete(date) {
          Some(diary) => Ok(diary),
          None => Err(String::from("No entry"))
        }
      },
      None => Err(String::from("No user"))
    }
  })
}

#[query]
pub fn load(principal:Principal, date:Date) -> Result<Diary, String> {
  USERS.with(|users| {
    let users = users.borrow();
    match users.get(&principal) {
      Some(user) => {
        if user.allowed(ic_cdk::caller()) {
          match user.load(date) {
            Some(diary) => Ok(diary),
            None => Err(String::from("No entry"))
          }
        } else {
          Err(String::from("Private"))
        }
      },
      None => Err(String::from("No user"))
    }
  })
}

#[query]
pub fn list(principal:Principal) -> Vec<Header> {
  USERS.with(|users| {
    let users = users.borrow();
    match users.get(&principal) {
      Some(user) => {
        if user.allowed(ic_cdk::caller()) {
          user.list()
            .into_iter()
            .map(|(date, title)| Header {
              date,
              title,
            })
            .collect::<Vec<Header>>()
        } else {
          // Private scope
          Vec::new()
        }
      },
      None => Vec::new()
    }
  })
}

#[pre_upgrade]
fn pre_upgrade() {
    USERS.with(|users| storage::stable_save((users,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    let (old_users,): (HashMap<Principal, User>,) = storage::stable_restore().unwrap();
    USERS.with(|users| *users.borrow_mut() = old_users);
}
use std::collections::BTreeMap;
use ic_cdk::api::time;
use candid::{Principal, CandidType, Deserialize};
use chrono::{TimeZone, Utc, LocalResult};

type Date = u32; // YYYYYMMDD
type EpocNano = u64; // Use ic_cdk::api::time() due to wasm not supported SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).as_secs()

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Diary {
  date: Date,
  title: String,
  content: String,
  created: EpocNano,
  updated: EpocNano,
}

// User
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct User {
  principal: Principal,
  pub public: bool,
  pub name: String,
  pub title: String,
  pub description: String,
  diaries: BTreeMap<Date, Diary>,
  created: EpocNano,
  updated: EpocNano,
}

impl User {
  pub fn new(principal:Principal, public:bool, name:String, title:String, description:String) -> Self {
    let now = time();
    User {
      principal,
      public,
      name,
      title,
      description,
      diaries: BTreeMap::new(),
      created: now,
      updated: now,
    }
  }
  
  pub fn update(&mut self, public:bool, name:String, title:String, description:String) -> () {
    self.public      = public;
    self.name        = name;
    self.title       = title;
    self.description = description;
    self.updated     = time();
  }

  pub fn save(&mut self, diary:Diary) -> Result<Diary, String> {
    if Utc.with_ymd_and_hms(diary.date as i32/10000, (diary.date/100)%100, diary.date%100, 0, 0, 0) == LocalResult::None {
      return Err(String::from("Invalid Date"));
    }

    let now = time();
    match self.diaries.get_mut(&diary.date) {
      Some(stored)=>{
        // Update
        if stored.updated == diary.updated {
          // OK
          stored.title   = diary.title;
          stored.content = diary.content;
          stored.updated = now;
          Ok(stored.clone())
        } else {
          // Error
          Err(String::from("Updated by other session"))
        }
      },
      None=>{
        // Insert
        let stored = Diary {
          // OK
          date: diary.date,
          title: diary.title,
          content: diary.content,
          created: now,
          updated: now,
        };
        self.diaries.insert(diary.date, stored.clone());
        Ok(stored)
      }
    }
  }

  pub fn delete(&mut self, date:Date) -> Option<Diary> {
    self.diaries.remove(&date)
  }

  pub fn allowed(&self, principal:Principal) -> bool {
    self.public || self.principal == principal
  }

  pub fn load(&self, date:Date) -> Option<Diary> {
    self.diaries.get(&date).cloned()
  }

  pub fn list(&self) -> BTreeMap<Date,String> {
    self.diaries
      .iter()
      .map(|(key, value)| {
        (key.clone(), value.title.clone())
      })
      .collect()
  }

  /* TODO
  fn listYear(&self, year:u32) -> BTreeMap<Date,String> {
    self.diaries
      .iter()
      .filter_map(|(key, value)| {
        if key / 10000 == year {
            Some((key.clone(), value.title.clone()))
        } else {
            None
        }
      })
      .collect()
  }

  fn listMonth(&self, year:u32, month:u32) -> BTreeMap<Date,String> {
    let yearmonth = year * 100 + month;
    self.diaries
      .iter()
      .filter_map(|(key, value)| {
        if key / 100 == yearmonth {
            Some((key.clone(), value.title.clone()))
        } else {
            None
        }
      })
      .collect()
  }
*/

/*
    let filtered_keys: Vec<_> = tree_map
    .iter()
    .filter(|(key, _value)| key.len() >= 6)
    .map(|(key, _value)| *key)
    .collect();
*/

}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let principal = Principal::from_text("aaaaa-aa").unwrap();
    let mut user = User::new(principal, String::from("user"), String::from("title"), String::from("description"));
    assert_eq!(principal,                   user.principal);
    assert_eq!(String::from("user"),        user.name);
    assert_eq!(String::from("title"),       user.title);
    assert_eq!(String::from("description"), user.description);
    assert!(user.diaries.is_empty());

    // save 1
    let saved1 = user.save(Diary {
      date: 20230827,
      title: String::from("title"),
      content: String::from("content"),
      created: 0,
      updated: 0,
    }).unwrap();
    assert_eq!(20230827,                saved1.date);
    assert_eq!(String::from("content"), saved1.content);
    assert_ne!(0,                       saved1.created);
    assert_eq!(saved1.updated,          saved1.created);
    assert_eq!(1, user.diaries.len());

    // save 2
    let saved2 = user.save(Diary {
      date: 20230828,
      title: String::from("title"),
      content: String::from("content"),
      created: 0,
      updated: 0,
    }).unwrap();
    assert_eq!(20230828,                saved2.date);
    assert_eq!(String::from("content"), saved2.content);
    assert_ne!(0,                       saved2.created);
    assert_eq!(saved2.updated,          saved2.created);
    assert_eq!(2, user.diaries.len());
  }

  
}

use ic_cdk::{
    export::{
        candid::{CandidType, Deserialize},
    }
};
use std::cell::RefCell;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
#[allow(non_snake_case)]
struct Homework {
    pub title: String,
    pub description: String,
    pub dueDate: i128,
    pub completed: bool
}

thread_local! {
    static HOMEWORK_DIARY: RefCell<Vec<Homework>> = RefCell::new(Vec::new());
}

#[ic_cdk_macros::update]
#[allow(non_snake_case)]
fn addHomework(homework: Homework) -> u128 {
    HOMEWORK_DIARY.with(|homework_diary| {
        let mut vec = homework_diary.borrow_mut();
        vec.push(homework);
        vec.len() as u128 - 1
    })
}

#[ic_cdk_macros::query]
#[allow(non_snake_case)]
fn getHomework(id: u128) -> Result<Homework, String> {
    HOMEWORK_DIARY.with(|homework_diary| {
        let vec = homework_diary.borrow(); 
        if id as usize >= vec.len() {
            Err(format!("Homework not found: {}", id))
        } else {
            Ok(vec[id as usize].clone())
        }
    })
}

#[ic_cdk_macros::update]
#[allow(non_snake_case)]
fn updateHomework(id: u128, homework: Homework) -> Result<(), String> {
    HOMEWORK_DIARY.with(|homework_diary| {
        let mut vec = homework_diary.borrow_mut();
        if id as usize >= vec.len() {
            Err(format!("Homework not found: {}", id))
        } else {
            let mut current = &mut vec[id as usize];
            current.title = homework.title;
            current.description = homework.description;
            current.dueDate = homework.dueDate;
            current.completed = homework.completed;
            Ok(())
        }
    })
}

#[ic_cdk_macros::update]
#[allow(non_snake_case)]
fn markAsCompleted(id: u128) -> Result<(), String> {
    HOMEWORK_DIARY.with(|homework_diary| {
        let mut vec = homework_diary.borrow_mut();
        if id as usize >= vec.len() {
            Err(format!("Homework not found: {}", id))
        } else {
            let mut current = &mut vec[id as usize];
            current.completed = true;
            Ok(())
        }
    })
}

#[ic_cdk_macros::update]
#[allow(non_snake_case)]
fn deleteHomework(id: u128) -> Result<(), String> {
    HOMEWORK_DIARY.with(|homework_diary| {
        let mut vec = homework_diary.borrow_mut();
        if id as usize >= vec.len() {
            Err(format!("Homework not found: {}", id))
        } else {
            vec.remove(id as usize);
            Ok(())
        }
    })
}


#[ic_cdk_macros::query]
#[allow(non_snake_case)]
fn getAllHomework() -> Vec<Homework> {
    HOMEWORK_DIARY.with(|homework_diary| {
        let vec = homework_diary.borrow();
        vec.clone()
    })
}


#[ic_cdk_macros::query]
#[allow(non_snake_case)]
fn getPendingHomework() -> Vec<Homework> {
    HOMEWORK_DIARY.with(|homework_diary| {
        homework_diary
            .borrow()
            .iter()
            .filter(|&homework|!homework.completed)
            .cloned()
            .collect::<Vec<Homework>>()
    })
}


#[ic_cdk_macros::query]
#[allow(non_snake_case)]
fn searchHomework(search_term: String) -> Vec<Homework> {
    HOMEWORK_DIARY.with(|homework_diary| {
        homework_diary
            .borrow()
            .iter()
            .filter(|&homework|
                homework.title.contains(&search_term) ||
                homework.description.contains(&search_term))
            .cloned()
            .collect::<Vec<Homework>>()
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    #[allow(non_snake_case)]
    fn addHomework() {
        reset();
        let homework = Homework {
            title: String::from("title"),
            description: String::from("description"),
            dueDate: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as i128,
            completed: false
        };
        let id = crate::addHomework(homework.clone());
        assert_eq!(0, id);

        let actual = crate::getHomework(id).unwrap();
        assert_eq!(homework.title, actual.title);
        assert_eq!(homework.description, actual.description);
        assert_eq!(homework.dueDate, actual.dueDate);
        assert_eq!(homework.completed, actual.completed);
    }

    #[test]
    #[allow(non_snake_case)]
    fn getHomework() {
        // test in addHomework(), updateHomeWork() for normal case

        // Error case
        reset();
        match crate::getHomework(0) {
            Ok(_) => panic!("Not expected"),
            Err(_) => {}
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn updateHomework() {
        reset();
        let id = crate::addHomework(Homework {
            title: String::from("title"),
            description: String::from("description"),
            dueDate: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as i128,
            completed: false
        });
        assert_eq!(0, id);

        // updated
        let updated = Homework {
            title: String::from("updated title"),
            description: String::from("updated description"),
            dueDate: 0,
            completed: true
        };
        crate::updateHomework(id, updated.clone()).unwrap();

        // matched
        let fetched = crate::getHomework(id).unwrap();
        assert_eq!(updated.title, fetched.title);
        assert_eq!(updated.description, fetched.description);
        assert_eq!(updated.dueDate, fetched.dueDate);
        assert_eq!(updated.completed, fetched.completed);
    }

    #[test]
    #[allow(non_snake_case)]
    fn markAsCompleted() {
        reset();
        let id = crate::addHomework( Homework {
            title: String::from("title"),
            description: String::from("description"),
            dueDate: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as i128,
            completed: false
        });
        assert_eq!(0, id);

        // updated
        crate::markAsCompleted(id).unwrap();
    }

    #[test]
    #[allow(non_snake_case)]
    fn deleteHomework() {
        reset();
        let homework = Homework {
            title: String::from("title"),
            description: String::from("description"),
            dueDate: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as i128,
            completed: false
        };
        let id = crate::addHomework(homework.clone());
        assert_eq!(0, id);
        crate::deleteHomework(id).unwrap();

        // Err case
        match crate::deleteHomework(id) {
            Ok(_) => panic!("Not expected"),
            Err(_) => {}
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn getAllHomework() {
        reset();
        crate::addHomework(Homework {
            title: String::from("completed"),
            description: String::from("description"),
            dueDate: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as i128,
            completed: true
        });
        crate::addHomework(Homework {
            title: String::from("pending"),
            description: String::from("description"),
            dueDate: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as i128,
            completed: false
        });

        // test for getPendingHomework()
        let pending = crate::getPendingHomework();
        assert_eq!(1, pending.len());
        assert_eq!("pending", pending[0].title);

        // completed
        crate::markAsCompleted(1).unwrap();
        let marked = crate::getHomework(0).unwrap();
        assert_eq!(true, marked.completed);
        assert_eq!(0, crate::getPendingHomework().len());
    }

    #[test]
    #[allow(non_snake_case)]
    fn searchHomework() {
        // test in getAllHomework()
        reset();
        crate::addHomework(Homework {
            title: String::from("ABC"),
            description: String::from("123"),
            dueDate: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as i128,
            completed: true
        });
        crate::addHomework(Homework {
            title: String::from("BCA"),
            description: String::from("231"),
            dueDate: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as i128,
            completed: false
        });
        crate::addHomework(Homework {
            title: String::from("CAB"),
            description: String::from("312"),
            dueDate: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as i128,
            completed: false
        });

        // test for searchHomework() with title matches
        let title_matched = crate::searchHomework(String::from("AB"));
        assert_eq!(2, title_matched.len());
        assert_eq!("ABC", title_matched[0].title);
        assert_eq!("CAB", title_matched[1].title);

        // test for searchHomework() with title matches
        let description_matched = crate::searchHomework(String::from("31"));
        assert_eq!(2, description_matched.len());
        assert_eq!("BCA", description_matched[0].title);
        assert_eq!("CAB", description_matched[1].title);

        // no match
        assert_eq!(0, crate::searchHomework(String::from("NO")).len());
    }

    fn reset() {
        HOMEWORK_DIARY.with(|homework_diary| homework_diary.borrow_mut().clear());
    }
}

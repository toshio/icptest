use candid::{CandidType, Principal};
use serde::Deserialize;
use std::cell::Cell;

thread_local! {
    static COUNTER: Cell<u64> = Cell::new(0);
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Counter {
    topic: String,
    value: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Subscriber {
    topic: String,
}

#[ic_cdk::update]
async fn setup_subscribe(publisher_id: Principal, topic: String) {
    let subscriber = Subscriber { topic };
    let _call_result: Result<(), _> =
        ic_cdk::call(publisher_id, "subscribe", (subscriber,)).await;
}

#[ic_cdk::update]
fn update_count(counter: Counter) {
    COUNTER.with(|c| {
        c.set(c.get() + counter.value);
    });
}

#[ic_cdk::query]
fn get_count() -> u64 {
    COUNTER.with(|c| {
        c.get()
    })
}
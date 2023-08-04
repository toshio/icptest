use std::cell::RefCell;

thread_local! {
    static VALUE: RefCell<String> = RefCell::default();
}

#[ic_cdk::update]
fn set(text: String) {
    VALUE.with(|value| {
        *value.borrow_mut() = text;
    });
}

#[ic_cdk::query]
fn get() -> String {
    VALUE.with(|value| {
        value.borrow().clone()
    })
}

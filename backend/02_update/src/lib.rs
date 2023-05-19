use std::cell::RefCell;

thread_local! {
    static VALUE: RefCell<String> = RefCell::default();
}

#[ic_cdk_macros::update]
fn set(text: String) {
    VALUE.with(|value| {
        *value.borrow_mut() = text;
    });
}

#[ic_cdk_macros::query]
fn get() -> String {
    VALUE.with(|value| {
        value.borrow().clone()
    })
}

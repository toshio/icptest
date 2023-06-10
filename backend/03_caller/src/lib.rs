#[ic_cdk_macros::query]
fn greet() -> String {
    let caller = ic_cdk::api::caller();
    format!("Hello, {}!", caller)
}

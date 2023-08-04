#[ic_cdk::query]
fn greet() -> String {
    let caller = ic_cdk::caller();
    format!("Hello, {}!", caller)
}

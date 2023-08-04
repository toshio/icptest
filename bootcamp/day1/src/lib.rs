use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<f64> = RefCell::new(0f64);
}

#[ic_cdk::update]
fn add(x: f64) -> f64 {
    COUNTER.with(|counter| {
        let mut c = counter.borrow_mut();
        *c += x;
        *c
    })
}

#[ic_cdk::update]
fn sub(x: f64) -> f64 {
    COUNTER.with(|counter| {
        let mut c = counter.borrow_mut();
        *c -= x;
        *c
    })
}

#[ic_cdk::update]
fn mul(x: f64) -> f64 {
    COUNTER.with(|counter| {
        let mut c = counter.borrow_mut();
        *c *= x;
        *c
    })
}

#[ic_cdk::update]
fn div(x: f64) -> Option<f64> {
    if x == 0.0 {
        None
    } else {
        COUNTER.with(|counter| {
            let mut c = counter.borrow_mut();
            *c /= x;
            Some(*c)
        })
    }
}

#[ic_cdk::update]
fn reset() {
    COUNTER.with(|counter| {
        *counter.borrow_mut() = 0.0
    })
}

#[ic_cdk::query]
fn see() -> f64 {
    COUNTER.with(|counter| {
        counter.borrow().clone()
    })
}

#[ic_cdk::update]
fn power(x: f64) -> f64 {
    COUNTER.with(|counter| {
        let mut c = counter.borrow_mut();
        *c = f64::powf(*c, x);
        *c
    })
}

#[ic_cdk::update]
fn sqrt() -> f64 {
    COUNTER.with(|counter| {
        let mut c = counter.borrow_mut();
        *c = f64::sqrt(*c);
        *c
    })
}

#[ic_cdk::update]
fn floor() -> i128 {
    COUNTER.with(|counter| {
        let mut c = counter.borrow_mut();
        *c = f64::floor(*c);
        *c as i128
    })
}


#[cfg(test)]
mod tests {
  #[test]
  fn add() {
    crate::reset();
    assert_eq!(1.0, crate::add(1.0));
    assert_eq!(3.0, crate::add(2.0));
    assert_eq!(0.0, crate::add(-3.0));
  }

  #[test]
  fn sub() {
    crate::reset();
    assert_eq!(-1.0, crate::sub(1.0));
    assert_eq!(-3.0, crate::sub(2.0));
    assert_eq!(0.0, crate::sub(-3.0));
  }

  #[test]
  fn mul() {
    crate::reset();
    crate::add(1.0);
    assert_eq!(2.0, crate::mul(2.0));
    assert_eq!(-6.0, crate::mul(-3.0));
    assert_eq!(3.0, crate::mul(-0.5));
  }

  #[test]
  fn div() {
    crate::reset();
    crate::add(1.0);
    assert_eq!(Some(2.0), crate::div(0.5));
    assert_eq!(Some(-1.0), crate::div(-2.0));
    assert_eq!(None, crate::div(-0.0));
  }

  #[test]
  fn reset() {
    crate::reset();
    crate::add(1.0);
    assert_eq!(1.0, crate::see());
    crate::reset();
    assert_eq!(0.0, crate::see());
  }

  #[test]
  fn see() {
    // test in reset()
  }

  #[test]
  fn power() {
    crate::reset();
    crate::add(2.0);
    assert_eq!(256.0, crate::power(8.0));
    assert_eq!(16.0, crate::power(0.5));
  }

  #[test]
  fn sqrt() {
    crate::reset();
    crate::add(256.0);
    assert_eq!(16.0, crate::sqrt());
    assert_eq!(4.0, crate::sqrt());
  }

  #[test]
  fn floor() {
    crate::reset();
    crate::add(123.45);
    assert_eq!(123, crate::floor());
    crate::sub(123.4);
    assert_eq!(-1, crate::floor());
  }
}

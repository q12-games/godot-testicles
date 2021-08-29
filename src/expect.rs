pub struct AssertionValue<T> {
  prefix: String,
  value: T,
}

pub type AssertionResult = Result<(), String>;

impl<T> AssertionValue<T> {
  pub fn new(prefix: String, v: T) -> Self {
    Self { prefix, value: v }
  }
}

impl AssertionValue<bool> {
  pub fn to_be_true(&self) -> AssertionResult {
    self.to_equal(true)
  }
  pub fn to_be_false(&self) -> AssertionResult {
    self.to_equal(false)
  }
}
impl<T: PartialEq + std::fmt::Debug> AssertionValue<T> {
  pub fn to_equal(&self, val: T) -> AssertionResult {
    if val == self.value {
      Ok(())
    } else {
      Err(format!(
        "{}\n - AssertionError: value {:?} does not equal {:?}",
        self.prefix, self.value, val
      ))
    }
  }
}
impl<T: euclid::approxeq::ApproxEq<T> + std::fmt::Debug> AssertionValue<T> {
  pub fn to_approx_equal(&self, val: T) -> AssertionResult {
    if val.approx_eq(&self.value) {
      Ok(())
    } else {
      Err(format!(
        "{}\n - AssertionError: value {:?} does not approx. equal {:?}",
        self.prefix, self.value, val
      ))
    }
  }
}

#[macro_export]
macro_rules! expect {
  ($val:expr) => {
    godot_testicles::AssertionValue::new(godot_testicles::get_path!(), $val)
  };
}

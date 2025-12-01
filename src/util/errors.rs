use std::fmt::Display;

#[derive(Debug)]
pub struct MyError(pub String);

impl<X: Display> From<X> for MyError {
	fn from(value: X) -> Self {
		MyError(format!("{}", value))
	}
}

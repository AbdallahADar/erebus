pub trait HashableValue {}
impl HashableValue for i64 {}
impl HashableValue for bool {}
impl HashableValue for String {}

// Did not work for what we intended
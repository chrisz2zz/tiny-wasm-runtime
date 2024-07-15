#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    I32(i32),
    I64(i64),
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::I32(value)
    }
}

impl From<Value> for i32 {
    fn from(value: Value) -> Self {
        match value {
            Value::I32(value) => value,
            _ => panic!("type mismatch"),
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}

impl std::ops::Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::I32(a), Value::I32(b)) => Value::I32(a + b),
            (Value::I64(a), Value::I64(b)) => Value::I64(a + b),
            _ => panic!("type mismatch"),
        }
    }
}

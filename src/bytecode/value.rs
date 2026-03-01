use crate::runtime::error::RuntimeError;

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::Boolean(_) => "boolean",
            Value::Nil => "nil",
            Value::String(_) => "string",
        }
    }

    pub fn negate(self, ip: usize) -> Result<Value, RuntimeError> {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            other => Err(RuntimeError::UnaryTypeError {
                op: "negate",
                got: other.type_name(),
                ip,
            }),
        }
    }

    pub fn add(self, rhs: Value, ip: usize) -> Result<Value, RuntimeError> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => {
                let mut result = String::new();
                result.push_str(&a);
                result.push_str(&b);
                Ok(Value::String(result))
            }
            (a, b) => Err(RuntimeError::BinaryTypeError {
                op: "+",
                a: a.type_name(),
                b: b.type_name(),
                ip,
            }),
        }
    }

    pub fn sub(self, rhs: Value, ip: usize) -> Result<Value, RuntimeError> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            (a, b) => Err(RuntimeError::BinaryTypeError {
                op: "-",
                a: a.type_name(),
                b: b.type_name(),
                ip,
            }),
        }
    }

    pub fn mul(self, rhs: Value, ip: usize) -> Result<Value, RuntimeError> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            (a, b) => Err(RuntimeError::BinaryTypeError {
                op: "*",
                a: a.type_name(),
                b: b.type_name(),
                ip,
            }),
        }
    }

    pub fn div(self, rhs: Value, ip: usize) -> Result<Value, RuntimeError> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
            (a, b) => Err(RuntimeError::BinaryTypeError {
                op: "/",
                a: a.type_name(),
                b: b.type_name(),
                ip,
            }),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Nil => write!(f, "nil"),
            Value::String(s) => write!(f, "{s}"),
        }
    }
}

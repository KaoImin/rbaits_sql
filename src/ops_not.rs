use crate::ops::Not;

use crate::ops::Value;

impl Not for Value {
    type Output = bool;

    fn op_not(self) -> Self::Output {
        match self {
            serde_json::Value::Bool(b) => { !b }
            _ => { true }
        }
    }
}

impl Not for &Value {
    type Output = bool;
    fn op_not(self) -> Self::Output {
        match self {
            serde_json::Value::Bool(b) => { !*b }
            _ => { true }
        }
    }
}

impl Not for &mut Value {
    type Output = bool;
    fn op_not(self) -> Self::Output {
        match self {
            serde_json::Value::Bool(b) => { !*b }
            _ => { true }
        }
    }
}
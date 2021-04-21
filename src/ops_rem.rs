use crate::Value;
use std::ops::Rem;
use crate::ops::AsProxy;


//serde
impl Rem<serde_json::Value> for Value {
    type Output = Value;
    fn rem(self, rhs: serde_json::Value) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() % rhs.as_i64().unwrap_or_default()).into_proxy()
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() % rhs.as_f64().unwrap_or_default()).into_proxy()
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() % rhs.as_u64().unwrap_or_default()).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}

impl Rem<&serde_json::Value> for Value {
    type Output = Value;
    fn rem(self, rhs: &serde_json::Value) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() % rhs.as_i64().unwrap_or_default()).into_proxy()
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() % rhs.as_f64().unwrap_or_default()).into_proxy()
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() % rhs.as_u64().unwrap_or_default()).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}

impl Rem<serde_json::Value> for &Value {
    type Output = Value;
    fn rem(self, rhs: serde_json::Value) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() % rhs.as_i64().unwrap_or_default()).into_proxy()
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() % rhs.as_f64().unwrap_or_default()).into_proxy()
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() % rhs.as_u64().unwrap_or_default()).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}

impl Rem<&serde_json::Value> for &Value {
    type Output = Value;
    fn rem(self, rhs: &serde_json::Value) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() % rhs.as_i64().unwrap_or_default()).into_proxy()
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() % rhs.as_f64().unwrap_or_default()).into_proxy()
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() % rhs.as_u64().unwrap_or_default()).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}

//value

impl Rem<Value> for Value {
    type Output = Value;
    fn rem(self, rhs: Value) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() % rhs.as_i64().unwrap_or_default()).into_proxy()
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() % rhs.as_f64().unwrap_or_default()).into_proxy()
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() % rhs.as_u64().unwrap_or_default()).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}

impl Rem<&Value> for Value {
    type Output = Value;
    fn rem(self, rhs: &Value) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() % rhs.as_i64().unwrap_or_default()).into_proxy()
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() % rhs.as_f64().unwrap_or_default()).into_proxy()
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() % rhs.as_u64().unwrap_or_default()).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}

impl Rem<Value> for &Value {
    type Output = Value;
    fn rem(self, rhs: Value) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() % rhs.as_i64().unwrap_or_default()).into_proxy()
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() % rhs.as_f64().unwrap_or_default()).into_proxy()
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() % rhs.as_u64().unwrap_or_default()).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}

impl Rem<&Value> for &Value {
    type Output = Value;
    fn rem(self, rhs: &Value) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() % rhs.as_i64().unwrap_or_default()).into_proxy()
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() % rhs.as_f64().unwrap_or_default()).into_proxy()
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() % rhs.as_u64().unwrap_or_default()).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}



fn rem_i64(value: &Value, other: i64) -> i64 {
    value.as_i64().unwrap_or_default() % other
}

fn rem_u64(value: &Value, other: u64) -> u64 {
    value.as_u64().unwrap_or_default() % other
}

fn rem_f64(value: &Value, other: f64) -> f64 {
    value.as_f64().unwrap_or_default() % other
}


fn rem_i64_value(value: &Value, other: i64) -> i64 {
    other % value.as_i64().unwrap_or_default()
}

fn rem_u64_value(value: &Value, other: u64) -> u64 {
    other % value.as_u64().unwrap_or_default()
}

fn rem_f64_value(value: &Value, other: f64) -> f64 {
    other % value.as_f64().unwrap_or_default()
}


macro_rules! impl_numeric_rem {
    ($($rem:ident,$rem_value:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl Rem<$ty> for Value {
                type Output = $return_ty;
                fn rem(self, other: $ty) -> Self::Output {
                    $rem(&self, other as _)
                }
            }

            impl Rem<Value> for $ty {
                type Output = $return_ty;
                fn rem(self, other: Value) -> Self::Output {
                    $rem_value(&other, self as _)
                }
            }

            impl Rem<&Value> for $ty {
                type Output = $return_ty;
                fn rem(self, other: &Value) -> Self::Output {
                    $rem_value(other, self as _)
                }
            }

            impl Rem<&mut Value> for $ty {
                type Output = $return_ty;
                fn rem(self, other: &mut Value) -> Self::Output {
                    $rem_value(other, self as _)
                }
            }

            impl<'a> Rem<$ty> for &'a Value {
                type Output = $return_ty;
                fn rem(self, other: $ty) -> Self::Output {
                    $rem(self, other as _)
                }
            }

            impl<'a> Rem<$ty> for &'a mut Value {
                type Output = $return_ty;
                fn rem(self, other: $ty) -> Self::Output {
                    $rem(self, other as _)
                }
            }
        )*)*
    }
}


impl_numeric_rem! {
    rem_i64,rem_i64_value[i8 i16 i32 i64 isize] -> i64
    rem_u64,rem_u64_value[u8 u16 u32 u64 usize] -> u64
    rem_f64,rem_f64_value[f32 f64] -> f64
}




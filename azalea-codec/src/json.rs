use serde_json::{Value, json};

use crate::{
    MapLike, Number, Ops,
    error::{ReadError, ReadResult, WriteError, WriteResult},
};

// https://github.com/Mojang/DataFixerUpper/blob/master/src/main/java/com/mojang/serialization/JsonOps.java
impl Ops for Value {
    type List = Vec<Value>;
    type Map = serde_json::Map<String, Value>;

    fn empty() -> Self {
        json!({})
    }

    fn get_number(&self) -> ReadResult<Number> {
        Ok(match self {
            Value::Number(n) => {
                if let Some(v) = n.as_i64() {
                    Number::Long(v)
                } else if let Some(v) = n.as_f64() {
                    Number::Double(v)
                } else {
                    return Err(ReadError::custom("number is too big"));
                }
            }
            Value::String(s) => {
                let Ok(n) = s.parse::<i32>() else {
                    return Err(ReadError::custom("not a number"));
                };
                Number::Int(n)
            }
            _ => return Err(ReadError::custom("not a number")),
        })
    }
    fn create_number(&self, n: Number) -> WriteResult<Self> {
        let n = if n.is_numeric_integer() {
            serde_json::Number::from_u128(n.into()).unwrap()
        } else {
            serde_json::Number::from_f64(n.into()).unwrap()
        };
        Ok(Value::Number(n))
    }

    fn get_boolean(&self) -> ReadResult<bool> {
        let Value::Bool(v) = self else {
            return Err(ReadError::custom("not a boolean"));
        };
        Ok(*v)
    }
    fn create_boolean(&self, v: bool) -> WriteResult<Self> {
        Ok(Value::Bool(v))
    }

    fn get_list(&self) -> ReadResult<Self::List> {
        let Value::Array(values) = self else {
            return Err(ReadError::custom("not an array"));
        };
        Ok(values.clone())
    }
    fn create_list(&self, list: &[Self]) -> WriteResult<Self> {
        Ok(Value::Array(list.to_owned()))
    }

    fn get_map(&self) -> ReadResult<Self::Map> {
        let Value::Object(values) = self else {
            return Err(ReadError::custom("not an object"));
        };
        Ok(values.clone())
    }

    fn create_map(&self, map: &[(Self, Self)]) -> WriteResult<Self> {
        let mut values = serde_json::Map::new();
        for (k, v) in map {
            let Ok(k) = k.get_string() else {
                return Err(WriteError::custom("map keys must be strings"));
            };
            values.insert(k, v.clone());
        }
        Ok(Value::Object(values))
    }

    fn get_string(&self) -> ReadResult<String> {
        self.as_str()
            .map(|s| s.to_owned())
            .ok_or_else(|| ReadError::custom("not a string"))
    }
    fn create_string(&self, v: &str) -> WriteResult<Self> {
        Ok(Value::String(v.to_owned()))
    }
}

impl MapLike<Value> for serde_json::Map<String, Value> {
    fn for_each(&self, mut cb: impl FnMut(&Value, &Value)) {
        self.iter()
            .for_each(|(k, v)| cb(&Value::String(k.to_owned()), v));
    }
}

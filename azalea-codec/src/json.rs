use serde_json::Value;

use crate::{
    Codec, MapLike, Number, Ops,
    error::{ReadError, ReadResult, WriteError, WriteResult},
};

// https://github.com/Mojang/DataFixerUpper/blob/master/src/main/java/com/mojang/serialization/JsonOps.java
impl Ops for Value {
    type List = Vec<Value>;
    type Map = serde_json::Map<String, Value>;

    fn get_number(&self) -> ReadResult<Number> {
        match self {
            Value::Number(n) => {
                if let Some(v) = n.as_i64() {
                    Ok(Number::Long(v))
                } else if let Some(v) = n.as_f64() {
                    Ok(Number::Double(v))
                } else {
                    return Err(ReadError::custom("not a valid codec number"));
                }
            }
            Value::String(s) => {
                if let Ok(n) = s.parse::<i32>() {
                    Ok(Number::Int(n))
                } else {
                    Err(ReadError::custom("not a number"))
                }
            }
            _ => Err(ReadError::custom("not a number")),
        }
    }
    fn create_number(n: Number) -> WriteResult<Self> {
        let n = if let Ok(n) = n.try_into() {
            serde_json::Number::from_i128(n).unwrap()
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
    fn create_boolean(v: bool) -> WriteResult<Self> {
        Ok(Value::Bool(v))
    }

    fn get_list(&self) -> ReadResult<Self::List> {
        let Value::Array(values) = self else {
            return Err(ReadError::custom("not an array"));
        };
        Ok(values.clone())
    }
    fn create_list(list: &[Self]) -> WriteResult<Self> {
        Ok(Value::Array(list.to_owned()))
    }

    fn get_map(&self) -> ReadResult<Self::Map> {
        let Value::Object(values) = self else {
            return Err(ReadError::custom("not an object"));
        };
        Ok(values.clone())
    }

    fn create_map(map: &[(Self, Self)]) -> WriteResult<Self> {
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
    fn create_string(v: &str) -> WriteResult<Self> {
        Ok(Value::String(v.to_owned()))
    }
}

impl<O: Ops> Codec<O> for serde_json::Map<String, Value> {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        let Ok(map) = ops.get_map() else {
            return Err(ReadError::custom("not a map"));
        };
        let map = map.to_values();
        let mut values = serde_json::Map::new();
        for (k, v) in map {
            let k = String::codec_read(&k)?;
            let v = Value::codec_read(&v)?;
            values.insert(k, v);
        }
        Ok(values)
    }
    fn codec_write(&self) -> WriteResult<O> {
        let mut map = Vec::new();
        for (k, v) in self {
            map.push((k.codec_write()?, v.codec_write()?));
        }
        O::create_map(&map)
    }
}
impl MapLike<Value> for serde_json::Map<String, Value> {
    fn for_each(&self, mut cb: impl FnMut(&Value, &Value)) {
        self.iter()
            .for_each(|(k, v)| cb(&Value::String(k.to_owned()), v));
    }
}

impl<O: Ops> Codec<O> for serde_json::Number {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        let n = ops.get_number()?;
        if let Ok(n) = n.try_into()
            && let Some(n) = Self::from_i128(n)
        {
            return Ok(n);
        }
        if let Some(n) = Self::from_f64(n.into()) {
            return Ok(n);
        }
        Err(ReadError::custom("not a number"))
    }
    fn codec_write(&self) -> WriteResult<O> {
        let n = if let Some(v) = self.as_i64() {
            Number::Long(v)
        } else if let Some(v) = self.as_f64() {
            Number::Double(v)
        } else {
            return Err(WriteError::custom("number is too big"));
        };
        n.codec_write()
    }
}

impl<O: Ops> Codec<O> for Value {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        if let Ok(v) = String::codec_read(ops) {
            return Ok(Value::String(v));
        }
        if let Ok(v) = serde_json::Number::codec_read(ops) {
            return Ok(Value::Number(v));
        }
        if let Ok(v) = bool::codec_read(ops) {
            return Ok(Value::Bool(v));
        }
        if let Ok(v) = serde_json::Map::<String, Value>::codec_read(ops) {
            return Ok(Value::Object(v));
        }
        if let Ok(v) = Vec::<Value>::codec_read(ops) {
            return Ok(Value::Array(v));
        }
        Err(ReadError::custom("unknown type"))
    }
    fn codec_write(&self) -> WriteResult<O> {
        todo!()
    }
}

use simd_json::{
    StaticNode,
    base::ValueAsScalar,
    owned::{Object, Value},
};

use crate::{
    Codec, Ctx, MapLike, Number, Ops,
    error::{ReadError, ReadResult, WriteError, WriteResult},
};

impl Ops for Value {
    type List = Vec<Value>;
    type Map = Object;

    fn get_number(&self, _ctx: Ctx<Self>) -> ReadResult<Number> {
        match self {
            Value::Static(s) => Ok(match *s {
                StaticNode::I64(n) => Number::from(n),
                StaticNode::U64(n) if let Ok(n) = Number::try_from(n) => n,
                StaticNode::F64(n) => Number::from(n),
                _ => return Err(ReadError::custom("not a valid codec number")),
            }),
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
    fn create_number(_ctx: Ctx<Self>, n: Number) -> WriteResult<Self> {
        let n = if let Ok(n) = n.try_into() {
            StaticNode::I64(n)
        } else {
            StaticNode::F64(n.into())
        };
        Ok(Value::Static(n))
    }

    fn get_boolean(&self, _ctx: Ctx<Self>) -> ReadResult<bool> {
        let Value::Static(StaticNode::Bool(v)) = self else {
            return Err(ReadError::custom("not a boolean"));
        };
        Ok(*v)
    }
    fn create_boolean(_ctx: Ctx<Self>, v: bool) -> WriteResult<Self> {
        Ok(Value::Static(StaticNode::Bool(v)))
    }

    fn get_list(&self, _ctx: Ctx<Self>) -> ReadResult<Self::List> {
        let Value::Array(values) = self else {
            return Err(ReadError::custom("not an array"));
        };
        Ok(*values.clone())
    }
    fn create_list(_ctx: Ctx<Self>, list: &[Self]) -> WriteResult<Self> {
        Ok(Value::Array(Box::new(list.to_owned())))
    }

    fn get_map(&self, _ctx: Ctx<Self>) -> ReadResult<Self::Map> {
        let Value::Object(values) = self else {
            return Err(ReadError::custom("not an object"));
        };
        Ok(*values.clone())
    }

    fn create_map(ctx: Ctx<Self>, map: &[(Self, Self)]) -> WriteResult<Self> {
        let mut values = Object::new();
        for (k, v) in map {
            let Ok(k) = k.get_string(ctx) else {
                return Err(WriteError::custom("map keys must be strings"));
            };
            values.insert(k, v.clone());
        }
        Ok(Value::Object(Box::new(values)))
    }

    fn get_string(&self, _ctx: Ctx<Self>) -> ReadResult<String> {
        self.as_str()
            .map(|s| s.to_owned())
            .ok_or_else(|| ReadError::custom("not a string"))
    }
    fn create_string(_ctx: Ctx<Self>, v: &str) -> WriteResult<Self> {
        Ok(Value::String(v.to_owned()))
    }
}

impl<O: Ops> Codec<O> for Object {
    fn codec_read(ops: &O, ctx: Ctx<O>) -> ReadResult<Self> {
        let Ok(map) = ops.get_map(ctx) else {
            return Err(ReadError::custom("not a map"));
        };
        let map = map.to_values();
        let mut values = Object::new();
        for (k, v) in map {
            let k = String::codec_read(&k, ctx)?;
            let v = Value::codec_read(&v, ctx)?;
            values.insert(k, v);
        }
        Ok(values)
    }
    fn codec_write(&self, ctx: Ctx<O>) -> WriteResult<O> {
        let mut map = Vec::new();
        for (k, v) in self {
            map.push((k.codec_write(ctx)?, v.codec_write(ctx)?));
        }
        O::create_map(ctx, &map)
    }
}
impl MapLike<Value> for simd_json::owned::Object {
    fn for_each(&self, mut cb: impl FnMut(&Value, &Value)) {
        self.iter()
            .for_each(|(k, v)| cb(&Value::String(k.to_owned()), v));
    }
}

impl<O: Ops> Codec<O> for Value {
    fn codec_read(ops: &O, ctx: Ctx<O>) -> ReadResult<Self> {
        let value = if let Ok(v) = String::codec_read(ops, ctx) {
            Value::String(v)
        } else if let Ok(n) = i64::codec_read(ops, ctx) {
            Value::Static(StaticNode::I64(n))
        } else if let Ok(n) = f64::codec_read(ops, ctx) {
            Value::Static(StaticNode::F64(n))
        } else if let Ok(n) = u64::codec_read(ops, ctx) {
            Value::Static(StaticNode::U64(n))
        } else if let Ok(v) = bool::codec_read(ops, ctx) {
            Value::Static(StaticNode::Bool(v))
        } else if let Ok(v) = Object::codec_read(ops, ctx) {
            Value::Object(Box::new(v))
        } else if let Ok(v) = Vec::<Value>::codec_read(ops, ctx) {
            Value::Array(Box::new(v))
        } else {
            return Err(ReadError::custom("unknown type"));
        };
        Ok(value)
    }
    fn codec_write(&self, _ctx: Ctx<O>) -> WriteResult<O> {
        todo!()
    }
}

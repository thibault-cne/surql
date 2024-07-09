use surrealdb::sql::{Function, Value};

pub struct Count {
    param: Option<Value>,
}

impl Count {
    pub fn new<V>(param: Option<V>) -> Self
    where
        V: Into<Value>,
    {
        Self {
            param: param.map(|v| v.into()),
        }
    }
}

impl From<Count> for Function {
    fn from(value: Count) -> Self {
        Function::Normal(
            "count".to_string(),
            value.param.map_or(Vec::new(), |v| vec![v]),
        )
    }
}

impl From<Count> for Value {
    fn from(value: Count) -> Self {
        Function::from(value).into()
    }
}

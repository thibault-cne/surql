use surrealdb::sql::{Function, Value};

pub struct Count {
    param: Option<Value>,
}

impl Count {
    pub fn new(param: Option<Value>) -> Self {
        Self { param }
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

use surrealdb::sql::{Function, Value};

pub struct Count {
    pub param: Option<Value>,
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

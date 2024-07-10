use surrealdb::{
    opt::IntoQuery,
    sql::{Function, Statement, Value},
};

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

impl IntoQuery for Count {
    fn into_query(self) -> surrealdb::Result<Vec<surrealdb::sql::Statement>> {
        Ok(vec![self.into()])
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

impl From<Count> for Statement {
    fn from(value: Count) -> Self {
        Statement::Value(value.into())
    }
}

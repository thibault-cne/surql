use surrealdb::sql::{Operator, Value};

#[derive(Clone, Debug)]
pub enum Expression {
    Unary { o: Operator, v: Value },
    Binary { l: Value, o: Operator, r: Value },
}

impl Expression {
    pub fn and<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::Binary {
            l: l.into(),
            o: Operator::And,
            r: r.into(),
        }
    }

    pub fn eq<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::Binary {
            l: l.into(),
            o: Operator::Equal,
            r: r.into(),
        }
    }

    pub fn ne<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::Binary {
            l: l.into(),
            o: Operator::NotEqual,
            r: r.into(),
        }
    }

    pub fn unary<V>(o: Operator, v: V) -> Self
    where
        V: Into<Value>,
    {
        Self::Unary { o, v: v.into() }
    }

    pub fn binary<L, R>(l: L, o: Operator, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::Binary {
            l: l.into(),
            o,
            r: r.into(),
        }
    }
}

impl From<Expression> for Value {
    fn from(value: Expression) -> Self {
        let expr = match value {
            Expression::Unary { o, v } => surrealdb::sql::Expression::Unary { o, v },
            Expression::Binary { l, o, r } => surrealdb::sql::Expression::Binary { l, o, r },
        };
        expr.into()
    }
}

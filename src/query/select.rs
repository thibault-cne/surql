use surrealdb::{
    opt::IntoQuery,
    sql::{
        Cond, Fetch, Fetchs, Field, Fields, Group, Groups, Idiom, Idioms, Limit, Order, Orders,
        Start, Value, Values,
    },
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SelectStatement {
    // `VALUE`keyword
    pub(crate) value: bool,
    // list of fields to select
    pub(crate) expr: Fields,
    // list of fields to omit
    pub(crate) omits: Option<Idioms>,
    // list of values to select from
    pub(crate) what: Values,
    // list of conditions to apply
    pub(crate) cond: Option<Cond>,
    // list of groups to apply
    pub(crate) group: Option<Groups>,
    pub(crate) order: Option<Orders>,
    // `LIMIT` keyword
    pub(crate) limit: Option<Limit>,
    // `FETCH` keyword
    pub(crate) fetch: Option<Fetchs>,
    pub(crate) start: Option<Start>,
}

impl SelectStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an expression to the select expression list.
    pub fn expr<T>(&mut self, expr: T) -> &mut Self
    where
        T: Into<Value>,
    {
        self.expr.0.push(Field::Single {
            expr: expr.into(),
            alias: None,
        });
        self
    }

    pub fn expr_as<T, U>(&mut self, expr: T, alias: U) -> &mut Self
    where
        T: Into<Value>,
        U: Into<Idiom>,
    {
        self.expr.0.push(Field::Single {
            expr: expr.into(),
            alias: Some(alias.into()),
        });
        self
    }

    /// Add a list of expressions to the select expression list.
    pub fn exprs<T>(&mut self, exprs: Vec<T>) -> &mut Self
    where
        T: Into<Field>,
    {
        self.expr.0.extend(exprs.into_iter().map(Into::into));
        self
    }

    /// Select value
    pub fn value(&mut self) -> &mut Self {
        self.value = true;
        self
    }

    pub fn what<T>(&mut self, from: T) -> &mut Self
    where
        T: Into<Value>,
    {
        self.what.0.push(from.into());
        self
    }

    pub fn limit<T>(&mut self, limit: T) -> &mut Self
    where
        T: Into<Limit>,
    {
        self.limit = Some(limit.into());
        self
    }

    pub fn group<T>(&mut self, group: T) -> &mut Self
    where
        T: Into<Group>,
    {
        match &mut self.group {
            Some(groups) => groups.0.push(group.into()),
            None => self.group = Some(Groups(vec![group.into()])),
        }
        self
    }

    pub fn cond<T>(&mut self, cond: T) -> &mut Self
    where
        T: Into<Value>,
    {
        match &mut self.cond {
            Some(conds) => {
                let expr = surrealdb::sql::Expression::Binary {
                    l: conds.0.clone(),
                    o: surrealdb::sql::Operator::And,
                    r: cond.into(),
                };
                self.cond = Some(Cond(expr.into()));
            }
            None => self.cond = Some(Cond(cond.into())),
        }
        self
    }

    pub fn order<T>(&mut self, order: T) -> &mut Self
    where
        T: Into<Order>,
    {
        match &mut self.order {
            Some(orders) => orders.0.push(order.into()),
            None => self.order = Some(Orders(vec![order.into()])),
        }
        self
    }

    pub fn start<T>(&mut self, start: T) -> &mut Self
    where
        T: Into<Start>,
    {
        self.start = Some(start.into());
        self
    }

    pub fn fetch<T>(&mut self, fetch: T) -> &mut Self
    where
        T: Into<Fetch>,
    {
        match &mut self.fetch {
            Some(fetchs) => fetchs.0.push(fetch.into()),
            None => self.fetch = Some(Fetchs(vec![fetch.into()])),
        }
        self
    }
}

impl IntoQuery for SelectStatement {
    fn into_query(self) -> surrealdb::Result<Vec<surrealdb::sql::Statement>> {
        Ok(vec![self.into()])
    }
}

impl From<SelectStatement> for surrealdb::sql::statements::SelectStatement {
    fn from(value: SelectStatement) -> Self {
        Self {
            expr: value.expr,
            omit: value.omits,
            what: value.what,
            cond: value.cond,
            group: value.group,
            order: value.order,
            limit: value.limit,
            fetch: value.fetch,
            start: value.start,
            ..Default::default()
        }
    }
}

impl From<SelectStatement> for surrealdb::sql::Statement {
    fn from(value: SelectStatement) -> Self {
        surrealdb::sql::Statement::Select(value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expression;

    #[test]
    fn simple_query() {
        let query: surrealdb::sql::statements::SelectStatement = SelectStatement::new()
            .expr("name")
            .expr("age")
            .what("users")
            .to_owned()
            .into();

        assert_eq!(
            query.to_string(),
            "SELECT 'name', 'age' FROM 'users'".to_string()
        );
    }

    #[test]
    fn where_clause() {
        let query: surrealdb::sql::statements::SelectStatement = SelectStatement::new()
            .expr("name")
            .expr("age")
            .what("users")
            .cond(Expression::eq("name", "toto"))
            .cond(Expression::eq("age", 18))
            .to_owned()
            .into();

        assert_eq!(
            query.to_string(),
            "SELECT 'name', 'age' FROM 'users' WHERE 'name' = 'toto' AND 'age' = 18".to_string()
        );
    }
}

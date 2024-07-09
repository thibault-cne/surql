use surrealdb::sql::{
    Cond, Dir, Field, Fields, Groups, Idiom, Limit, Orders, Splits, Start, Table, Tables,
};

#[derive(Clone, Debug, Default)]
pub struct Graph {
    dir: Dir,
    expr: Fields,
    what: Tables,
    cond: Option<Cond>,
    split: Option<Splits>,
    group: Option<Groups>,
    order: Option<Orders>,
    limit: Option<Limit>,
    start: Option<Start>,
    alias: Option<Idiom>,
}

impl Graph {
    pub fn graph_in() -> Self {
        Self {
            dir: Dir::In,
            ..Default::default()
        }
    }

    pub fn graph_out() -> Self {
        Self {
            dir: Dir::Out,
            ..Default::default()
        }
    }

    pub fn graph_both() -> Self {
        Self {
            dir: Dir::Both,
            ..Default::default()
        }
    }

    pub fn expr<T>(&mut self, expr: T) -> &mut Self
    where
        T: Into<Field>,
    {
        self.expr.0.push(expr.into());
        self
    }

    pub fn exprs<T>(&mut self, exprs: T) -> &mut Self
    where
        T: Into<Fields>,
    {
        self.expr = exprs.into();
        self
    }

    pub fn what<T>(&mut self, what: T) -> &mut Self
    where
        T: Into<Table>,
    {
        self.what.0.push(what.into());
        self
    }
}

impl From<Graph> for surrealdb::sql::Graph {
    fn from(value: Graph) -> Self {
        Self {
            dir: value.dir,
            expr: value.expr,
            what: value.what,
            cond: value.cond,
            split: value.split,
            group: value.group,
            order: value.order,
            limit: value.limit,
            start: value.start,
            alias: value.alias,
        }
    }
}

impl From<Graph> for surrealdb::sql::Part {
    fn from(value: Graph) -> Self {
        Self::Graph(value.into())
    }
}

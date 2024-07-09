use surrealdb::sql::{Dir, Thing};

use crate::table::Tables;

pub struct Edge {
    dir: Dir,
    from: Thing,
    what: Tables,
}

impl Edge {
    pub fn dir_in<T, TB>(from: T, what: TB) -> Self
    where
        T: Into<Thing>,
        TB: Into<Tables>,
    {
        Self {
            dir: Dir::In,
            from: from.into(),
            what: what.into(),
        }
    }

    pub fn dir_out<T, TB>(from: T, what: TB) -> Self
    where
        T: Into<Thing>,
        TB: Into<Tables>,
    {
        Self {
            dir: Dir::Out,
            from: from.into(),
            what: what.into(),
        }
    }

    pub fn dir_both<T, TB>(from: T, what: TB) -> Self
    where
        T: Into<Thing>,
        TB: Into<Tables>,
    {
        Self {
            dir: Dir::Both,
            from: from.into(),
            what: what.into(),
        }
    }
}

impl From<Edge> for surrealdb::sql::Edges {
    fn from(value: Edge) -> Self {
        Self {
            dir: value.dir,
            from: value.from,
            what: value.what.into(),
        }
    }
}

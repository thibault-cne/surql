use std::ops::Deref;
use std::str;

use surrealdb::sql::Ident;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct Tables(pub Vec<Table>);

impl From<Table> for Tables {
    fn from(v: Table) -> Self {
        Tables(vec![v])
    }
}

impl From<Vec<Table>> for Tables {
    fn from(v: Vec<Table>) -> Self {
        Self(v)
    }
}

impl Deref for Tables {
    type Target = Vec<Table>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Tables> for surrealdb::sql::Tables {
    fn from(v: Tables) -> Self {
        Self(v.0.into_iter().map(Into::into).collect())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct Table(pub String);

impl From<String> for Table {
    fn from(v: String) -> Self {
        Self(v)
    }
}

impl From<&str> for Table {
    fn from(v: &str) -> Self {
        Self::from(String::from(v))
    }
}

impl From<Ident> for Table {
    fn from(v: Ident) -> Self {
        Self(v.0)
    }
}

impl Deref for Table {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Table> for surrealdb::sql::Table {
    fn from(v: Table) -> Self {
        Self(v.0)
    }
}

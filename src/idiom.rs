use surrealdb::sql::{Part, Value};

#[derive(Clone, Debug, Default)]
pub struct Idiom(Vec<Part>);

impl Idiom {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push<P>(&mut self, part: P) -> &mut Self
    where
        P: Into<Part>,
    {
        self.0.push(part.into());
        self
    }
}

impl From<String> for Idiom {
    fn from(v: String) -> Self {
        Self(vec![Part::from(v)])
    }
}

impl From<Vec<Part>> for Idiom {
    fn from(v: Vec<Part>) -> Self {
        Self(v)
    }
}

impl From<&[Part]> for Idiom {
    fn from(v: &[Part]) -> Self {
        Self(v.to_vec())
    }
}
impl From<Part> for Idiom {
    fn from(v: Part) -> Self {
        Self(vec![v])
    }
}

impl From<Idiom> for surrealdb::sql::Idiom {
    fn from(value: Idiom) -> Self {
        Self(value.0)
    }
}

impl From<Idiom> for Value {
    fn from(value: Idiom) -> Self {
        Self::Idiom(value.into())
    }
}

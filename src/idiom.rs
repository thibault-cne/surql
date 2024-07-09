use surrealdb::sql::Part;

#[derive(Clone, Debug, Default)]
pub struct Idioms(Vec<Idiom>);

impl From<Vec<Idiom>> for Idioms {
    fn from(v: Vec<Idiom>) -> Self {
        Self(v)
    }
}

impl From<&[Idiom]> for Idioms {
    fn from(v: &[Idiom]) -> Self {
        Self(v.to_vec())
    }
}

impl From<Idioms> for surrealdb::sql::Idioms {
    fn from(value: Idioms) -> Self {
        Self(value.0.into_iter().map(Into::into).collect())
    }
}

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

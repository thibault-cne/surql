use crate::idiom::Idiom;

pub struct Order {
    pub order: Idiom,
    pub random: bool,
    pub collate: bool,
    pub numeric: bool,
    /// true if the direction is ascending
    pub direction: bool,
}

impl Order {
    pub fn asc<F>(field: F) -> Order
    where
        F: Into<Idiom>,
    {
        Order {
            order: field.into(),
            random: false,
            collate: false,
            numeric: false,
            direction: true,
        }
    }

    pub fn desc<F>(field: F) -> Order
    where
        F: Into<Idiom>,
    {
        Order {
            order: field.into(),
            random: false,
            collate: false,
            numeric: false,
            direction: false,
        }
    }
}

impl From<Order> for surrealdb::sql::Order {
    fn from(value: Order) -> Self {
        Self {
            order: value.order.into(),
            random: value.random,
            collate: value.collate,
            numeric: value.numeric,
            direction: value.direction,
        }
    }
}

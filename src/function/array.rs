use surrealdb::sql::{Function, Value};

pub enum ArrayFunction {
    Add(Value, Value),
    All(Value),
    Any(Value),
    At(Value, usize),
    Append(Value, Value),
    BooleanAnd(Value, Value),
    BooleanOr(Value, Value),
    BooleanXor(Value, Value),
    BooleanNot(Value),
    Combine(Value, Value),
    Complement(Value, Value),
    Clump(Value, Value),
    Concat(Value, Value),
    Difference(Value, Value),
    Distinct(Value),
    FindIndex(Value, usize),
    FilterIndex(Value, usize),
    First(Value),
    Flatten(Value),
    Group(Value),
    Insert(Value, Value, usize),
    Intersect(Value, Value),
    Join(Value, String),
    Last(Value),
    Len(Value),
    LogicalAnd(Value, Value),
    LogicalOr(Value, Value),
    LogicalXor(Value, Value),
    Max(Value),
    Matches(Value, Value),
    Min(Value),
    Pop(Value),
    Prepend(Value, Value),
    Push(Value, Value),
    Remove(Value, usize),
    Reverse(Value),
    Shuffle(Value),
    SortAsc(Value),
    SortDesc(Value),
    Slice(Value, usize, usize),
    Transpose(Value, Value),
    Union(Value, Value),
}

impl ArrayFunction {
    pub fn add<A, V>(array: A, value: V) -> Self
    where
        A: Into<Value>,
        V: Into<Value>,
    {
        Self::Add(array.into(), value.into())
    }

    pub fn all<A>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::All(array.into())
    }

    pub fn any<A>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::Any(array.into())
    }

    pub fn at<A>(array: A, index: usize) -> Self
    where
        A: Into<Value>,
    {
        Self::At(array.into(), index)
    }

    pub fn append<A, V>(array: A, value: V) -> Self
    where
        A: Into<Value>,
        V: Into<Value>,
    {
        Self::Append(array.into(), value.into())
    }

    pub fn boolean_and<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::BooleanAnd(l.into(), r.into())
    }

    pub fn boolean_or<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::BooleanOr(l.into(), r.into())
    }

    pub fn boolean_xor<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::BooleanXor(l.into(), r.into())
    }

    pub fn boolean_not<A>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::BooleanNot(array.into())
    }

    pub fn combine<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::Combine(l.into(), r.into())
    }

    pub fn complement<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::Complement(l.into(), r.into())
    }

    pub fn clump<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::Clump(l.into(), r.into())
    }

    pub fn concat<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::Concat(l.into(), r.into())
    }

    pub fn difference<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::Difference(l.into(), r.into())
    }

    pub fn distinct<A>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::Distinct(array.into())
    }

    pub fn find_index<A>(array: A, index: usize) -> Self
    where
        A: Into<Value>,
    {
        Self::FindIndex(array.into(), index)
    }

    pub fn filter_index<A>(array: A, index: usize) -> Self
    where
        A: Into<Value>,
    {
        Self::FilterIndex(array.into(), index)
    }

    pub fn first<A>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::First(array.into())
    }

    pub fn flatten<A>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::Flatten(array.into())
    }

    pub fn group<A>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::Group(array.into())
    }

    pub fn insert<A, V>(array: A, value: V, index: usize) -> Self
    where
        A: Into<Value>,
        V: Into<Value>,
    {
        Self::Insert(array.into(), value.into(), index)
    }

    pub fn intersect<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::Intersect(l.into(), r.into())
    }

    pub fn join<A, V>(array: A, value: V) -> Self
    where
        A: Into<Value>,
        V: Into<String>,
    {
        Self::Join(array.into(), value.into())
    }

    pub fn last<A>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::Last(array.into())
    }

    pub fn len<A>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::Len(array.into())
    }

    pub fn logical_and<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::LogicalAnd(l.into(), r.into())
    }

    pub fn logical_or<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::LogicalOr(l.into(), r.into())
    }

    pub fn logical_xor<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::LogicalXor(l.into(), r.into())
    }

    pub fn max<A, F>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::Max(array.into())
    }

    pub fn matches<A, V>(array: A, value: V) -> Self
    where
        A: Into<Value>,
        V: Into<Value>,
    {
        Self::Matches(array.into(), value.into())
    }

    pub fn min<A, F>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::Min(array.into())
    }

    pub fn pop<A>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::Pop(array.into())
    }

    pub fn prepend<A, V>(array: A, value: V) -> Self
    where
        A: Into<Value>,
        V: Into<Value>,
    {
        Self::Prepend(array.into(), value.into())
    }

    pub fn push<A, V>(array: A, value: V) -> Self
    where
        A: Into<Value>,
        V: Into<Value>,
    {
        Self::Push(array.into(), value.into())
    }

    pub fn remove<A>(array: A, index: usize) -> Self
    where
        A: Into<Value>,
    {
        Self::Remove(array.into(), index)
    }

    pub fn reverse<A>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::Reverse(array.into())
    }

    pub fn shuffle<A>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::Shuffle(array.into())
    }

    pub fn sort_asc<A>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::SortAsc(array.into())
    }

    pub fn sort_desc<A>(array: A) -> Self
    where
        A: Into<Value>,
    {
        Self::SortDesc(array.into())
    }

    pub fn slice<A>(array: A, start: usize, end: usize) -> Self
    where
        A: Into<Value>,
    {
        Self::Slice(array.into(), start, end)
    }

    pub fn transpose<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::Transpose(l.into(), r.into())
    }

    pub fn union<L, R>(l: L, r: R) -> Self
    where
        L: Into<Value>,
        R: Into<Value>,
    {
        Self::Union(l.into(), r.into())
    }
}

impl From<ArrayFunction> for Function {
    fn from(value: ArrayFunction) -> Self {
        use ArrayFunction::*;

        match value {
            Add(array, value) => Function::Normal("array::add".to_string(), vec![array, value]),
            All(array) => Function::Normal("array::all".to_string(), vec![array]),
            Any(array) => Function::Normal("array::any".to_string(), vec![array]),
            At(array, index) => {
                Function::Normal("array::at".to_string(), vec![array, index.into()])
            }
            Append(array, value) => {
                Function::Normal("array::append".to_string(), vec![array, value])
            }
            BooleanAnd(array, value) => {
                Function::Normal("array::boolean_and".to_string(), vec![array, value])
            }
            BooleanOr(array, value) => {
                Function::Normal("array::boolean_or".to_string(), vec![array, value])
            }
            BooleanXor(array, value) => {
                Function::Normal("array::boolean_xor".to_string(), vec![array, value])
            }
            BooleanNot(array) => Function::Normal("array::boolean_not".to_string(), vec![array]),
            Combine(array, value) => {
                Function::Normal("array::combine".to_string(), vec![array, value])
            }
            Complement(array, value) => {
                Function::Normal("array::complement".to_string(), vec![array, value])
            }
            Clump(array, value) => Function::Normal("array::clump".to_string(), vec![array, value]),
            Concat(array, value) => {
                Function::Normal("array::concat".to_string(), vec![array, value])
            }
            Difference(array, value) => {
                Function::Normal("array::difference".to_string(), vec![array, value])
            }
            Distinct(array) => Function::Normal("array::distinct".to_string(), vec![array]),
            FindIndex(array, value) => {
                Function::Normal("array::find_index".to_string(), vec![array, value.into()])
            }
            FilterIndex(array, value) => {
                Function::Normal("array::filter_index".to_string(), vec![array, value.into()])
            }
            First(array) => Function::Normal("array::first".to_string(), vec![array]),
            Flatten(array) => Function::Normal("array::flatten".to_string(), vec![array]),
            Group(array) => Function::Normal("array::group".to_string(), vec![array]),
            Insert(array, value, index) => Function::Normal(
                "array::insert".to_string(),
                vec![array, value, index.into()],
            ),
            Intersect(array, value) => {
                Function::Normal("array::intersect".to_string(), vec![array, value])
            }
            Join(array, value) => {
                Function::Normal("array::join".to_string(), vec![array, value.into()])
            }
            Last(array) => Function::Normal("array::last".to_string(), vec![array]),
            Len(array) => Function::Normal("array::len".to_string(), vec![array]),
            LogicalAnd(array, value) => {
                Function::Normal("array::logical_and".to_string(), vec![array, value])
            }
            LogicalOr(array, value) => {
                Function::Normal("array::logical_or".to_string(), vec![array, value])
            }
            LogicalXor(array, value) => {
                Function::Normal("array::logical_xor".to_string(), vec![array, value])
            }
            Max(array) => Function::Normal("array::max".to_string(), vec![array]),
            Matches(array, value) => {
                Function::Normal("array::matches".to_string(), vec![array, value])
            }
            Min(array) => Function::Normal("array::min".to_string(), vec![array]),
            Pop(array) => Function::Normal("array::pop".to_string(), vec![array]),
            Prepend(array, value) => {
                Function::Normal("array::prepend".to_string(), vec![array, value])
            }
            Push(array, value) => Function::Normal("array::push".to_string(), vec![array, value]),
            Remove(array, value) => {
                Function::Normal("array::remove".to_string(), vec![array, value.into()])
            }
            Reverse(array) => Function::Normal("array::reverse".to_string(), vec![array]),
            Shuffle(array) => Function::Normal("array::shuffle".to_string(), vec![array]),
            SortAsc(array) => Function::Normal("array::sort::asc".to_string(), vec![array]),
            SortDesc(array) => Function::Normal("array::sort::desc".to_string(), vec![array]),
            Slice(array, start, end) => Function::Normal(
                "array::slice".to_string(),
                vec![array, start.into(), end.into()],
            ),
            Transpose(array, value) => {
                Function::Normal("array::transpose".to_string(), vec![array, value])
            }
            Union(array, value) => Function::Normal("array::union".to_string(), vec![array, value]),
        }
    }
}

impl From<ArrayFunction> for Value {
    fn from(value: ArrayFunction) -> Self {
        Function::from(value).into()
    }
}

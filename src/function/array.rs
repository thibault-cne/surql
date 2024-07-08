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

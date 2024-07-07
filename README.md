# surql

Surql is a rust library to build surrealQL queries.

This library is build on top of the [surreal](https://github.com/surrealdb/surrealdb) official client library. It provides a more ergonomic way to build queries.

## Example

```rust
use surql::query::SelectStatement;

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
```

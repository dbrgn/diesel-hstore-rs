use std::collections::HashMap;

#[derive(Queryable)]
pub struct Animal {
    pub id: i32,
    pub name: String,
    pub attributes: HashMap<String, String>,
}

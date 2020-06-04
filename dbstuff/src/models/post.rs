use bigdecimal::BigDecimal;
use crate::schema::post;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "post"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub value: BigDecimal,
}

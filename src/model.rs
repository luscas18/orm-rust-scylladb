use scylla::{FromRow, SerializeRow, ValueList};
use uuid::Uuid;

#[derive(Debug,SerializeRow,FromRow,ValueList)]
pub struct  Book{
    pub id:Uuid,
    pub name:String,
    pub description:String,
    pub price:f64,
    pub quantity:i32
}

#[derive(SerializeRow)]
pub struct  SelectBook{
    pub id:Uuid
}
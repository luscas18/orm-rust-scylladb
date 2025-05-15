// model.rs
use uuid::Uuid;
use scylla::{FromRow, SerializeRow, ValueList};
use scylla::IntoTypedRows;
use orm_macro::ScyllaOrm;  // Importa a macro ScyllaOrm

#[derive(Debug, SerializeRow, FromRow, ValueList, ScyllaOrm)] // Deriva a macro ScyllaOrm para gerar os métodos
#[orm(table = "books", keyspace = "books_db", primary_key = "id")] // Atributos que configuram a tabela, keyspace e chave primária
pub struct Book {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: i32,
}

#[derive(SerializeRow)]
pub struct SelectBook {
    pub id: Uuid,
}

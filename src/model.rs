// model.rs
use uuid::Uuid;
use scylla::{FromRow, SerializeRow, ValueList, IntoTypedRows}; // Import the trait to enable the `into_typed` method
use orm_macro::ScyllaOrm;  // Importa a macro ScyllaOrm
use scylla::statement::Statement;
#[derive(Debug, SerializeRow, FromRow, ValueList,ScyllaOrm)] // Deriva a macro ScyllaOrm para gerar os métodos
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

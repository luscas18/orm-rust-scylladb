mod db;
mod model;
mod result;

use model::Book;
use tokio;
use uuid::Uuid;
use crate::db::create_session;
use crate::result::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let uri = "127.0.0.1:9042";
    let session = create_session(&uri).await?;
    println!("Conectado ao Cassandra!");

    // Cria o keyspace e a tabela, se não existirem
    Book::create_schema(&session).await?;

    // Cria um novo livro
    let key = Uuid::new_v4();
    let book = Book {
        id: key,
        name: "Dom Casmurro".to_string(),
        description: "Obra de Machado de Assis".to_string(),
        price: 49.90,
        quantity: 20,
    };

    // Insere o livro
    Book::insert(&session, book).await?;
    println!("Livro inserido!");

    // Busca o livro
    let livros = Book::select_by_id(&session, key).await?;
    println!("Livro(s) encontrado(s): {:?}", livros);

    Ok(())
}
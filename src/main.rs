mod db;
mod model;
mod result;

use db::{add_book, create_session, initialize, select_book};
use model::{Book, SelectBook};
use tokio::select;
use uuid::Uuid;
use crate::result::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let uri = "127.0.0.1:9042";
    let session = create_session(&uri).await?;
    initialize(&session).await;
    println!("Conectado");

    let key = Uuid::new_v4();
    let book = Book {
        id: key.clone(),
        name: String::from("Teste"),
        description:  String::from("Teste"),
        price: 1000.0,
        quantity: 100,
    };

    add_book(&session, book).await.unwrap();
    let select= SelectBook{
        id:key.clone()
    };
    let result = select_book(&session, select).await.unwrap();
    println!("Resultado: {:?}", result);

    Ok(())
}
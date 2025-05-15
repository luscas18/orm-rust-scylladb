// use scylla::{Session, SessionBuilder};
// use scylla::IntoTypedRows;
// use anyhow::Result;
// use crate::Book;
// use crate::model::SelectBook;


// pub async fn create_session<E>(uri: &str) -> Result<Session, dyn Eq> {
//     SessionBuilder::new()
//         .known_node(uri)
//         .build()
//         .await
//         .map_err(From::from)
// }

// static CREATE_KEYSPACE_QUERY: &str = r#"
//     CREATE KEYSPACE IF NOT EXISTS books_db
//     WITH REPLICATION = {
//         'class' : 'SimpleStrategy',
//         'replication_factor' : 1
//     };
// "#;

// static CREATE_BOOKS_TABLE_QUERY: &str = r#"
//    CREATE TABLE IF NOT EXISTS books_db.books (
//     id UUID,
//     name TEXT,
//     description TEXT,
//     price DOUBLE,
//     quantity INT,
//     PRIMARY KEY (id)
// );
// "#;

// static ADD_BOOK_QUERY: &str = r#"
//     INSERT INTO books_db.books (id, name, description, price, quantity)
//     VALUES (?, ?, ?, ?, ?);
// "#;

// static SELECT_BOOK_QUERY: &str = r#"
//     SELECT * FROM books_db.books
//         WHERE id=?
// "#;

// async fn create_keyspace<E: std::convert::From<scylla::transport::errors::QueryError>>(session: &Session) -> Result<(), E> {
//     session
//         .query(CREATE_KEYSPACE_QUERY, ())
//         .await
//         .map(|_| ())
//         .map_err(From::from)
// }

// async fn create_books_table<E: std::convert::From<scylla::transport::errors::QueryError>>(session: &Session) -> Result<(), E> {
//     session
//         .query(CREATE_BOOKS_TABLE_QUERY, ())
//         .await
//         .map(|_| ())
//         .map_err(From::from)
// }

// pub async fn initialize<E>(session: &Session) -> Result<(), E> {
//     create_keyspace(session).await?;
//     create_books_table(session).await?;
//     Ok(())
// }

// pub async fn add_book<E: std::convert::From<scylla::transport::errors::QueryError>>(session: &Session, book: Book) -> Result<(), E> {
//     session
//         .query(ADD_BOOK_QUERY, book)
//         .await
//         .map(|_| ())
//         .map_err(From::from)
// }

// pub async fn select_book<E>(session: &Session, select_book: SelectBook) -> Result<Vec<Book, dyn Eq>> {
//     session
//         .query(SELECT_BOOK_QUERY, select_book)
//         .await?
//         .rows()
//         .unwrap_or_default()
//         .into_typed::<Book>()
//         .map(|row_result| row_result.map_err(|e| e.into())) // converte FromRowError -> Box<dyn Error>
//         .collect()
// }


use scylla::{Session, SessionBuilder};
use scylla::IntoTypedRows;
use anyhow::Result;

use crate::Book;
use crate::model::SelectBook;

pub async fn create_session(uri: &str) -> Result<Session> {
    SessionBuilder::new()
        .known_node(uri)
        .build()
        .await
        .map_err(From::from)
}

static CREATE_KEYSPACE_QUERY: &str = r#"
    CREATE KEYSPACE IF NOT EXISTS books_db
    WITH REPLICATION = {
        'class' : 'SimpleStrategy',
        'replication_factor' : 1
    };
"#;

static CREATE_BOOKS_TABLE_QUERY: &str = r#"
   CREATE TABLE IF NOT EXISTS books_db.books (
    id UUID,
    name TEXT,
    description TEXT,
    price DOUBLE,
    quantity INT,
    PRIMARY KEY (id)
);
"#;

static ADD_BOOK_QUERY: &str = r#"
    INSERT INTO books_db.books (id, name, description, price, quantity)
    VALUES (?, ?, ?, ?, ?);
"#;

static SELECT_BOOK_QUERY: &str = r#"
    SELECT * FROM books_db.books
        WHERE id = ?;
"#;

async fn create_keyspace(session: &Session) -> Result<()> {
    session.query(CREATE_KEYSPACE_QUERY, ()).await?;
    Ok(())
}

async fn create_books_table(session: &Session) -> Result<()> {
    session.query(CREATE_BOOKS_TABLE_QUERY, ()).await?;
    Ok(())
}

pub async fn initialize(session: &Session) -> Result<()> {
    create_keyspace(session).await?;
    create_books_table(session).await?;
    Ok(())
}

pub async fn add_book(session: &Session, book: Book) -> Result<()> {
    session.query(ADD_BOOK_QUERY, book).await?;
    Ok(())
}

pub async fn select_book(session: &Session, select_book: SelectBook) -> Result<Vec<Book>> {
    let rows = session
        .query(SELECT_BOOK_QUERY, select_book)
        .await?
        .rows()
        .unwrap_or_default();

    let books: Vec<Book> = rows
        .into_typed::<Book>()
        .map(|row| row.map_err(Into::into))
        .collect::<Result<_, anyhow::Error>>()?;

    Ok(books)
}

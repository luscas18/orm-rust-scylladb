use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta, NestedMeta, Lit};
use scylla::statement::Statement;
#[proc_macro_derive(ScyllaOrm, attributes(orm))]
pub fn scylla_orm(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    let attrs = &input.attrs;
    let mut table_name = name.to_string().to_lowercase();
    let mut keyspace = "books_db".to_string();
    let mut primary_key = "id".to_string();

    for attr in attrs {
        if attr.path.is_ident("orm") {
            if let Ok(Meta::List(meta)) = attr.parse_meta() {
                for nested in meta.nested {
                    if let NestedMeta::Meta(Meta::NameValue(name_value)) = nested {
                        if let Some(ident) = name_value.path.get_ident() {
                            match ident.to_string().as_str() {
                                "table" => {
                                    if let Lit::Str(lit_str) = &name_value.lit {
                                        table_name = lit_str.value();
                                    }
                                }
                                "keyspace" => {
                                    if let Lit::Str(lit_str) = &name_value.lit {
                                        keyspace = lit_str.value();
                                    }
                                }
                                "primary_key" => {
                                    if let Lit::Str(lit_str) = &name_value.lit {
                                        primary_key = lit_str.value();
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    let expanded = quote! {
        impl #name {
            pub async fn create_schema(session: &scylla::Session) -> crate::result::Result<()> {
                let create_keyspace_query = format!(
                    "CREATE KEYSPACE IF NOT EXISTS {} WITH replication = {{'class': 'SimpleStrategy', 'replication_factor': 3}};",
                    #keyspace
                );
                let mut ks_stmt = scylla::Statement::new(create_keyspace_query, ());
                ks_stmt.set_consistency(scylla::statement::Consistency::One);
                session.query(ks_stmt, &[]).await?;

                let create_table_query = format!(
                    "CREATE TABLE IF NOT EXISTS {}.{} (
                        {} UUID PRIMARY KEY,
                        name TEXT,
                        description TEXT,
                        price DOUBLE,
                        quantity INT
                    );",
                    #keyspace,
                    #table_name,
                    #primary_key
                );
                let mut tbl_stmt = scylla::Statement::new(create_table_query, ());
                tbl_stmt.set_consistency(scylla::statement::Consistency::One);
                session.query(tbl_stmt, &[]).await?;

                Ok(())
            }

            pub async fn insert(session: &scylla::Session, item: #name) -> crate::result::Result<()> {
                let query = format!(
                    "INSERT INTO {}.{} ({}) VALUES ({});",
                    #keyspace,
                    #table_name,
                    "id, name, description, price, quantity",
                    "?, ?, ?, ?, ?"
                );
                let mut stmt = scylla::Statement::new(query, item);
                stmt.set_consistency(scylla::statement::Consistency::One);
                session.query(stmt, &[]).await.map(|_| ()).map_err(From::from)
            }

            pub async fn select_by_id(session: &scylla::Session, id: uuid::Uuid) -> crate::result::Result<Vec<#name>> {
                let query = format!(
                    "SELECT * FROM {}.{} WHERE {} = ?;",
                    #keyspace,
                    #table_name,
                    #primary_key
                );
                let mut stmt = scylla::Statement::new(query, (id,));
                stmt.set_consistency(scylla::statement::Consistency::One);
                session.query(stmt, &[]).await?
                    .rows()
                    .unwrap_or_default()
                    .into_typed::<#name>()
                    .map(|row_result| row_result.map_err(|e| e.into()))
                    .collect()
            }
        }
    };

    TokenStream::from(expanded)
}

mod mapping; // Garante que o módulo mapping seja compilado

use mapping::attributes::{column, table};
use mapping::registry::{MAPPING_REGISTRY, get_table_info, register_mapping};
use std::collections::HashMap;

#[derive(Debug)]
struct TestProduct {
    id: i32,
    name: String,
    price: f64,
}

fn main() {
    println!("--- Iniciando registro na main ---");

    // Registrar o mapeamento para TestProduct
    register_mapping::<TestProduct>(
        table("products_from_main"),
        vec![
            ("id", column("product_id")),
            ("name", column("product_name")),
            ("price", column("product_price")),
        ],
        Some("id"),
    );

    println!("--- Registro concluído na main ---");

    // Tentar acessar o registro e imprimir seu conteúdo
    println!("--- Conteúdo do MAPPING_REGISTRY na main ---");
    let registry = MAPPING_REGISTRY.get();
    match registry {
        Some(reg) => {
            for (type_id, table_info) in reg.iter() {
                println!("Type ID: {:?}", type_id);
                println!("  Table Name: {}", table_info.name);
                println!("  Primary Key: {:?}", table_info.primary_key);
                println!("  Columns:");
                for (field_name, column_info) in table_info.columns.iter() {
                    println!(
                        "    Field: {}, Column Name: {}, Is Primary Key: {}",
                        field_name, column_info.name, column_info.is_primary_key
                    );
                }
                println!("---");
            }
        }
        None => {
            println!("O MAPPING_REGISTRY não foi inicializado.");
        }
    }

    // Tentar obter informações de tabela
    println!("--- Tentando obter TableInfo na main ---");
    match get_table_info::<TestProduct>() {
        Some(info) => {
            println!("Table Info para TestProduct na main: {:?}", info);
        }
        None => {
            println!("Não foi possível obter TableInfo para TestProduct na main.");
        }
    }
    println!("Hello, world from main!");
}

pub mod mapping;
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::any::TypeId;
    use crate::mapping::registry::{register_mapping, get_table_info, MAPPING_REGISTRY};
    use crate::mapping::attributes::{table, column};
    use std::sync::OnceLock;

    #[derive(Debug)]
    struct TestProduct {
        id: i32,
        name: String,
        price: f64,
    }

    #[test]
    fn test_registry_output() {
        // Registrar o mapeamento para TestProduct
        register_mapping::<TestProduct>(
            table("products"),
            vec![
                ("id", column("product_id")),
                ("name", column("product_name")),
                ("price", column("product_price")),
            ],
            Some("id"),
        );

        // Acessar o registro (inicializando-o se não estiver)
        let registry = MAPPING_REGISTRY.get_or_init(|| HashMap::new());

        // Imprimir o conteúdo do registro
        println!("--- Conteúdo do MAPPING_REGISTRY ---");
        for (type_id, table_info) in registry.iter() {
            println!("Type ID: {:?}", type_id);
            println!("  Table Name: {}", table_info.name);
            println!("  Primary Key: {:?}", table_info.primary_key);
            println!("  Columns:");
            for (field_name, column_info) in table_info.columns.iter() {
                println!("    Field: {}, Column Name: {}, Is Primary Key: {}", field_name, column_info.name, column_info.is_primary_key);
            }
            println!("---");
        }

        // Você também pode verificar se as informações foram registradas corretamente usando asserts
        let product_table_info = get_table_info::<TestProduct>().unwrap();
        assert_eq!(product_table_info.name, "products");
        assert_eq!(product_table_info.columns.len(), 3);
        assert_eq!(product_table_info.primary_key, Some("product_id"));
    }
}
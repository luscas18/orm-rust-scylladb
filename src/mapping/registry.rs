use std::collections::HashMap;
use std::any::TypeId;
use crate::mapping::attributes::{Table, Column};
use std::sync::OnceLock;

type MappingRegistry = HashMap<TypeId, TableInfo>;

static MAPPING_REGISTRY: OnceLock<MappingRegistry> = OnceLock::new();

#[derive(Debug)]
pub struct TableInfo {
    pub name: &'static str,
    pub columns: HashMap<&'static str, ColumnInfo>,
    pub primary_key: Option<&'static str>,
}

#[derive(Debug)]
pub struct ColumnInfo {
    pub name: &'static str,
    pub is_primary_key: bool,
}

impl TableInfo {
    pub fn new(name: &'static str) -> Self {
        TableInfo {
            name,
            columns: HashMap::new(),
            primary_key: None,
        }
    }
}

pub fn get_mapping_registry() -> &'static MappingRegistry {
    MAPPING_REGISTRY.get_or_init(|| HashMap::new())
}

pub fn register_mapping<T: 'static>(table: Table, columns: Vec<(&'static str, Column)>, primary_key: Option<&'static str>) {
    let type_id = TypeId::of::<T>();
    let registry = get_mapping_registry();
    let mut table_info = TableInfo::new(table.name);

    for (field_name, col) in columns {
        table_info.columns.insert(col.name, ColumnInfo { name: col.name, is_primary_key: primary_key.map_or(false, |pk| pk == field_name) });
        if let Some(pk) = primary_key {
            if pk == field_name {
                table_info.primary_key = Some(col.name);
            }
        }
    }

    // Obtain a mutable reference to the OnceLock's value (only safe because it's initialized once)
    let registry_mut = MAPPING_REGISTRY.get_or_init(|| HashMap::new());
    unsafe {
        let mut_ref = (registry_mut as *const HashMap<TypeId, TableInfo> as *mut HashMap<TypeId, TableInfo>);
        (*mut_ref).insert(type_id, table_info);
    }
}

pub fn get_table_info<T: 'static>() -> Option<&'static TableInfo> {
    let type_id = TypeId::of::<T>();
    get_mapping_registry().get(&type_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mapping::attributes::{table, column};

    #[derive(Debug)]
    struct TestUser {
        id: i32,
        name: String,
        email: String,
    }

    #[test]
    fn test_register_and_get_mapping() {
        register_mapping::<TestUser>(
            table("test_users"),
            vec![
                ("id", column("user_id")),
                ("name", column("user_name")),
                ("email", column("user_email")),
            ],
            Some("id"),
        );

        let table_info = get_table_info::<TestUser>().unwrap();
        assert_eq!(table_info.name, "test_users");
        assert_eq!(table_info.columns.len(), 3);
        assert_eq!(table_info.columns.get("user_id").unwrap().name, "user_id");
        assert!(table_info.columns.get("user_id").unwrap().is_primary_key);
        assert_eq!(table_info.columns.get("user_name").unwrap().name, "user_name");
        assert!(!table_info.columns.get("user_name").unwrap().is_primary_key);
        assert_eq!(table_info.columns.get("user_email").unwrap().name, "user_email");
        assert!(!table_info.columns.get("user_email").unwrap().is_primary_key);
        assert_eq!(table_info.primary_key, Some("user_id"));
    }

    #[test]
    fn test_register_without_primary_key() {
        register_mapping::<TestUser>(
            table("test_users"),
            vec![
                ("id", column("user_id")),
                ("name", column("user_name")),
                ("email", column("user_email")),
            ],
            None,
        );

        let table_info = get_table_info::<TestUser>().unwrap();
        assert_eq!(table_info.primary_key, None);
        assert!(!table_info.columns.get("user_id").unwrap().is_primary_key);
    }
}
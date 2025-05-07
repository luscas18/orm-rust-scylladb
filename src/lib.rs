pub mod mapping;

#[cfg(test)]
mod tests {
    use crate::mapping::registry::{register_mapping, get_table_info};
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
#[derive(Debug, Clone, Copy)]
pub struct Table {
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Column {
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct PrimaryKey {}

// "Construtores" simples para usar como atributos
pub const table: fn(&'static str) -> Table = |name| Table { name };
pub const column: fn(&'static str) -> Column = |name| Column { name };
pub const primary_key: fn() -> PrimaryKey = || PrimaryKey {};
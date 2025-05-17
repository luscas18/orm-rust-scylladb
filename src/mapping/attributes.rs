#[derive(Debug, Clone, Copy)]
pub struct TABLE {
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct COLUMN {
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct PrimaryKey {}

// "Construtores" simples para usar como atributos
pub const TABLE: fn(&'static str) -> TABLE = |name| TABLE { name };
pub const COLUMN: fn(&'static str) -> COLUMN = |name| COLUMN { name };
pub const PRIMARY_KEY: fn() -> PrimaryKey = || PrimaryKey {};
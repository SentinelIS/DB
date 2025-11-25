use crate::parser::Column;

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
}

#[derive(Debug, Clone)]
pub struct Row {
    pub values: Vec<String>,
}

pub struct Catalog {
    tables: Vec<Table>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog {
            tables: Vec::new(),
        }
    }

    pub fn create_table(&mut self, name: String, columns: Vec<Column>) -> Result<(), String> {
        // Check if table already exists
        if self.tables.iter().any(|t| t.name == name) {
            return Err(format!("Table '{}' already exists", name));
        }

        let table = Table {
            name,
            columns,
            rows: Vec::new(),
        };
        self.tables.push(table);
        Ok(())
    }

    pub fn find_table_mut(&mut self, name: &str) -> Option<&mut Table> {
        self.tables.iter_mut().find(|t| t.name == name)
    }

    pub fn find_table(&self, name: &str) -> Option<&Table> {
        self.tables.iter().find(|t| t.name == name)
    }

    pub fn list_tables(&self) -> Vec<&str> {
        self.tables.iter().map(|t| t.name.as_str()).collect()
    }
}

pub struct QueryEngine {
    catalog: Catalog,
}

impl QueryEngine {
    pub fn new() -> Self {
        QueryEngine {
            catalog: Catalog::new(),
        }
    }

    pub fn execute_create_table(&mut self, name: String, columns: Vec<Column>) -> Result<(), String> {
        self.catalog.create_table(name, columns)
    }

    pub fn execute_insert(&mut self, table: String, values: Vec<String>) -> Result<(), String> {
        let table = self
            .catalog
            .find_table_mut(&table)
            .ok_or_else(|| format!("Table '{}' does not exist", table))?;

        // Validate column count
        if values.len() != table.columns.len() {
            return Err(format!(
                "Column count mismatch: expected {}, got {}",
                table.columns.len(),
                values.len()
            ));
        }

        table.rows.push(Row { values });
        Ok(())
    }

    pub fn execute_select(&self, table: String) -> Result<Vec<Row>, String> {
        let table = self
            .catalog
            .find_table(&table)
            .ok_or_else(|| format!("Table '{}' does not exist", table))?;

        Ok(table.rows.clone())
    }

    pub fn get_table_schema(&self, table: &str) -> Option<&Table> {
        self.catalog.find_table(table)
    }
}
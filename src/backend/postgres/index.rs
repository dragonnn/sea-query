use super::*;

impl IndexBuilder for PostgresQueryBuilder {
    fn prepare_index_create_statement(&self, create: &IndexCreateStatement, sql: &mut SqlWriter) {
        write!(sql, "CREATE ").unwrap();
        if create.unique {
            write!(sql, "UNIQUE ").unwrap();
        }
        write!(sql, "INDEX ").unwrap();

        if let Some(name) = &create.index.name {
            write!(sql, "\"{}\" ", name).unwrap();
        }

        write!(sql, "ON ").unwrap();
        if let Some(table) = &create.table {
            table.prepare(sql, '"');
        }

        if let Some(index_type) = &create.index_type {
            write!(sql, " USING {}", match index_type {
                IndexType::BTree => "BTREE".to_owned(),
                IndexType::FullText => "GIN".to_owned(),
                IndexType::Hash => "HASH".to_owned(),
                IndexType::Custom(custom) => custom.to_string(),
            }).unwrap();
        }

        write!(sql, " (").unwrap();
        create.index.columns.iter().fold(true, |first, col| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            col.prepare(sql, '"');
            false
        });
        write!(sql, ")").unwrap();
    }

    fn prepare_index_drop_statement(&self, drop: &IndexDropStatement, sql: &mut SqlWriter) {
        write!(sql, "DROP INDEX ").unwrap();
        if let Some(name) = &drop.index.name {
            write!(sql, "\"{}\"", name).unwrap();
        }
    }
}
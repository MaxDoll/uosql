
use super::super::meta::{Table};
use super::super::{Engine, Error};
use std::fs::OpenOptions;
use super::super::super::parse::ast;
use super::super::types::SqlType;

pub struct FlatFile<'a> {
    table: Table<'a>,
}

impl<'a> FlatFile<'a> {
    pub fn new<'b>(table: Table<'b>) -> FlatFile<'b> {
        println!("Hallo");
        FlatFile { table: table }
    }
}

impl<'a> Drop for FlatFile<'a> {
    fn drop(&mut self) {
        println!("Tschüss");
    }
}

impl<'a> Engine for FlatFile<'a> {
    fn create_table(&mut self) -> Result<(), Error> {
        let mut _file = try!(OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.table.get_table_data_path()));
        Ok(())
    }

    fn table(&self) -> &Table {
        &self.table
    }

    /// Insert values from data into rows of the table
    fn insert_row(&mut self, data: &[Option<ast::DataSrc>])
                  -> Result<(), Error> {

        // Open table data file
        let mut file = try!(OpenOptions::new()
                            .write(true)
                            .append(true)
                            .open(&self.table.get_table_data_path()));

        let defaults = [ast::DataSrc::Int(0),
                        ast::DataSrc::Bool(0),
                        ast::DataSrc::String("l".to_string()),
                        ast::DataSrc::String("o".to_string())];

        // Iterate over given columns data and the meta data
        // simultaneously and get either the given data or a
        // defaul type
        for (d, meta) in data.iter().zip(self.table().columns()) {
            // Entry contains default or given value
            let entry = d.as_ref().unwrap_or(match meta.sql_type {
                SqlType::Int => &defaults[0],
                SqlType::Bool => &defaults[1],
                SqlType::Char(_) => &defaults[2],
                SqlType::VarChar(_) => &defaults[3],
            });

            // Try to encode the data entry into the table file
            // (appends to end of file)
            try!(meta.sql_type.encode_into(&mut file, entry));
        }

        Ok(())
    }
}

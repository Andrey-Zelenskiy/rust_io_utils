// Copyright Andrey Zelenskiy, 2024
use toml;
use serde::Deserialize;
use std::fs;

/* ----------------------------------------- */
/* Methods for loading data from .toml files */
/* ----------------------------------------- */

// Open a config.toml file and save the data as a toml::Value
pub fn load_toml(filename: &str) -> toml::Table {
    // Read the contents of the file
    let contents = fs::read_to_string(filename).expect(
        "Problem opening the file: {filename}",
    );
    // Save the data to toml::Table
    let data_value = contents.parse::<toml::Value>().unwrap();
    data_value
        .as_table()
        .expect("{filename} should contain a table-type data.")
        .clone()
}

// Populate a struct with the data from the sub-table of the config file
pub trait FromConfig<'a>: Deserialize<'a> {
    fn from_config(
        config: &toml::Table,
        table_name: &str,
        ) -> Self { 

        // Create a toml::Table with only the relevant part of the config
        let table = config[table_name]
            .as_table()
            .expect("Expected a non-null sub-table {table_name}.")
            .clone();

        // Populate the structure
        table.try_into::<Self>()
            .expect(
                "Failed to initialize the structure for sub-table \
                {table_name}.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn file_not_found() {
        load_toml("this_file_doesnt_exist.txt");
    }
}

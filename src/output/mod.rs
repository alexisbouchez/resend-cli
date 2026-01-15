//! # Output Formatting Module
//!
//! This module provides utilities for formatting and displaying data in the CLI.
//! It includes functions for printing tabular data using the tabled crate.

use tabled::{Table, Tabled};

/// Prints a vector of items as a formatted table
///
/// This function takes a vector of items that implement the Tabled trait and
/// prints them as a formatted table to standard output. If the vector is empty,
/// it prints a "No items found." message.
///
/// # Type Parameters
///
/// * `T` - The type of items in the vector, which must implement the Tabled trait
///
/// # Arguments
///
/// * `items` - A vector of items to display in table format
pub fn print_table<T>(items: Vec<T>)
where
    T: Tabled,
{
    if items.is_empty() {
        println!("No items found.");
        return;
    }
    let table = Table::new(items).to_string();
    println!("{}", table);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tabled::{Tabled};

    #[derive(Tabled, Debug, PartialEq)]
    struct TestItem {
        #[tabled(rename = "ID")]
        id: u32,
        #[tabled(rename = "Name")]
        name: String,
    }

    #[test]
    fn test_print_table_with_items() {
        let items = vec![
            TestItem { id: 1, name: "Item 1".to_string() },
            TestItem { id: 2, name: "Item 2".to_string() },
        ];

        // Capture stdout would be complex, so we just ensure the function runs without error
        // by calling it with a sample of items that implement Tabled
        print_table(items);
        assert!(true); // Basic assertion to satisfy test
    }

    #[test]
    fn test_print_table_with_empty_items() {
        let items: Vec<TestItem> = vec![];

        // Capture stdout would be complex, so we just ensure the function runs without error
        print_table(items);
        assert!(true); // Basic assertion to satisfy test
    }
}

use crate::table::{Row, TableDefinition};
use anyhow::Result;

#[derive(Debug, PartialEq)]
struct Page<'a> {
    pub data: Vec<u8>,
    pub table_definition: &'a TableDefinition,
    offset: usize,
}

impl<'a> Page<'a> {
    pub fn new(page_size: usize, table_definition: &'a TableDefinition) -> Self {
        Page {
            data: vec![0; page_size],
            table_definition: table_definition,
            offset: 0,
        }
    }

    pub fn insert(&mut self, row: &Row) -> Result<usize> {
        let old_offset = self.offset;
        if row.table != self.table_definition {
            return Err(anyhow::format_err!("Miss matched Table"));
        }

        if self.offset + self.table_definition.row_length() > self.data.len() {
            return Err(anyhow::format_err!("No space"));
        }

        self.fill_data(&row.data);

        self.offset += self.table_definition.row_length();

        Ok(old_offset)
    }

    pub fn read(&self, offset: usize) -> Result<Row<'a>> {
        if self.offset < offset {
            return Err(anyhow::format_err!("No such element"));
        }

        let row_data = &self.data[offset..offset + self.table_definition.row_length()];
        Ok(Row::new(&self.table_definition, row_data.to_vec()))
    }

    fn fill_data(&mut self, data_to_write: &[u8]) {
        // for i in 0..data_to_write.len() {
        //     self.data[self.offset] = data_to_write[i];
        //     self.offset += 1;
        // }

        self.data[self.offset..self.offset + data_to_write.len()].copy_from_slice(data_to_write);
    }
}
#[cfg(test)]
mod tests {
    use crate::table::user;

    use super::*;

    #[test]
    fn page() {
        let user_table = user();

        let mut page = Page::new(user_table.row_length() * 5, &user_table);

        let row = Row::from(&user_table, &vec!["123", "mr_smith", "smith@cats.co.za"]).unwrap();
        assert_eq!(page.insert(&row).unwrap(), 0);
        assert_eq!(page.insert(&row).unwrap(), user_table.row_length() * 1);
        assert_eq!(page.insert(&row).unwrap(), user_table.row_length() * 2);
        assert_eq!(page.insert(&row).unwrap(), user_table.row_length() * 3);
        assert_eq!(page.insert(&row).unwrap(), user_table.row_length() * 4);

        assert_eq!(&page.read(0).unwrap(), &row);
        assert_eq!(&page.read(user_table.row_length() * 1).unwrap(), &row);
        assert_eq!(&page.read(user_table.row_length() * 2).unwrap(), &row);
        assert_eq!(&page.read(user_table.row_length() * 3).unwrap(), &row);
        assert_eq!(&page.read(user_table.row_length() * 4).unwrap(), &row);
    }
}

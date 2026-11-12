use std::collections::HashMap;

use anyhow::{Result, anyhow};

#[derive(Debug, PartialEq)]
pub struct TableDefinition {
    pub name: String,
    pub columns: Vec<Column>,
}

impl TableDefinition {
    pub fn row_length(&self) -> usize {
        self.columns.iter().map(|col| col.data_type.len()).sum()
    }
}

#[derive(Debug, PartialEq)]
pub enum DataType {
    Boolean,
    Integer,
    String(usize),
}

impl DataType {
    pub const TRUE: [u8; 4] = [0, 0, 0, 1];
    pub const FALSE: [u8; 4] = [0, 0, 0, 0];

    pub fn len(&self) -> usize {
        match self {
            DataType::Boolean => 1,
            DataType::Integer => 4,
            DataType::String(length) => length.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
}

impl Column {
    pub fn map_bytes_to_string(&self, data: &[u8]) -> String {
        assert_eq!(self.data_type.len(), data.len());
        match self.data_type {
            DataType::Boolean => {
                if data[0] == 0 {
                    "false".to_string()
                } else {
                    "true".to_string()
                }
            }
            DataType::Integer => i32::from_le_bytes(data[..4].try_into().unwrap()).to_string(),
            DataType::String(_) => str::from_utf8(data).unwrap().to_string(),
        }
    }

    pub fn map_string_to_bytes(&self, data: &str) -> Result<Vec<u8>> {
        match self.data_type {
            DataType::Boolean => {
                if data == "false" {
                    Ok(DataType::FALSE.to_vec())
                } else {
                    Ok(DataType::TRUE.to_vec())
                }
            }
            DataType::Integer => {
                let i = data.parse::<i32>()?;
                Ok(vec![
                    i as u8,
                    (i >> 8) as u8,
                    (i >> 16) as u8,
                    (i >> 24) as u8,
                ])
            }
            DataType::String(length) => {
                let mut vec = data.as_bytes().to_vec();
                match vec.len() {
                    l if length >= l => {
                        vec.resize(length as usize, 0u8);
                        Ok(vec)
                    }
                    other => Err(anyhow!(
                        "Invalid Length: expected '{length}', found '{other}'"
                    )),
                }
            }
        }
    }
}

pub fn user() -> TableDefinition {
    TableDefinition {
        columns: vec![
            Column {
                data_type: DataType::Integer,
                name: "id".to_string(),
            },
            Column {
                data_type: DataType::String(32),
                name: "username".to_string(),
            },
            Column {
                data_type: DataType::String(255),
                name: "email".to_string(),
            },
        ],
        name: "user".to_string(),
    }
}

#[derive(Debug, PartialEq)]
pub struct Row<'a> {
    pub table: &'a TableDefinition,
    pub data: Vec<u8>,
}

impl<'a> Row<'a> {
    pub fn new(table: &'a TableDefinition, data: Vec<u8>) -> Row<'a> {
        Row {
            table: table,
            data: data,
        }
    }

    pub fn from(table: &'a TableDefinition, args: &Vec<&str>) -> Result<Row<'a>> {
        let mut data = vec![0u8; table.row_length()];
        let mut offset: usize = 0;
        let columns = &table.columns;
        for (i, col) in columns.iter().enumerate() {
            let end = offset + col.data_type.len();
            let column_data = col.map_string_to_bytes(args[i])?;
            data.splice(offset..end, column_data);
            offset = end;
        }

        Ok(Row {
            table: table,
            data: data,
        })
    }

    pub fn to_string_map(&self) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        let mut offset: usize = 0;
        let columns = &self.table.columns;
        for col in columns {
            let end = offset + col.data_type.len();
            map.insert(
                col.name.clone(),
                col.map_bytes_to_string(&self.data[offset..end].to_vec()),
            );
            offset = end;
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let user = user();

        let row = Row::from(&user, &vec!["123", "mr_smith", "smith@cats.co.za"]).unwrap();
        let deserialized_row = &row.to_string_map();

        assert_eq!(deserialized_row.get("id").unwrap(), "123");
        assert_eq!(
            deserialized_row.get("username").unwrap(),
            "mr_smith\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
        );
        assert_eq!(
            deserialized_row.get("email").unwrap(),
            "smith@cats.co.za\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
        );
    }
}

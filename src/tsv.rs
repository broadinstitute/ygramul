use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Lines, Read};
use crate::error::Error;

pub(crate) enum Type {
    String, Float
}

pub(crate) enum Value {
    String(String),
    Float(f64)
}

pub(crate) struct Column {
    key: String,
    tpe: Type
}

pub(crate) struct Row {
    strings: BTreeMap<String, String>,
    floats: BTreeMap<String, f64>
}
pub(crate) struct TsvReader<R: Read> {
    lines: Lines<BufReader<R>>,
    col_map: Vec<Option<usize>>,
    cols: Vec<Column>
}

impl Type {
    pub(crate) fn parse(&self, value: &str) -> Result<Value, Error> {
        match self {
            Type::String => Ok(Value::String(value.to_string())),
            Type::Float => Ok(Value::Float(value.parse::<f64>()?))
        }
    }
}

impl Column {
    pub(crate) fn new(key: &str, tpe: Type) -> Column {
        Column { key: key.to_string(), tpe }
    }
}

impl Row {
    pub(crate) fn new() -> Row {
        Row { strings: BTreeMap::new(), floats: BTreeMap::new() }
    }
    pub(crate) fn add_string(&mut self, key: &str, value: &str) {
        self.strings.insert(key.to_string(), value.to_string());
    }
    pub(crate) fn add_float(&mut self, key: &str, value: f64) {
        self.floats.insert(key.to_string(), value);
    }
    pub(crate) fn add_value(&mut self, key: &str, value: Value) {
        match value {
            Value::String(string) => self.add_string(key, &string),
            Value::Float(float) => self.add_float(key, float)
        }
    }
    pub(crate) fn remove_string(&mut self, key: &str) -> Option<String> {
        self.strings.remove(key)
    }
    pub(crate) fn remove_float(&mut self, key: &str) -> Option<f64> {
        self.floats.remove(key)
    }
    pub(crate) fn remove_string_or_error(&mut self, key: &str) -> Result<String, Error> {
        self.remove_string(key).ok_or(Error::from(format!("Missing string for '{}'", key)))
    }
    pub(crate) fn remove_float_or_error(&mut self, key: &str) -> Result<f64, Error> {
        self.remove_float(key).ok_or(Error::from(format!("Missing float for '{}'", key)))
    }
}

impl<R: Read> TsvReader<R> {
    pub(crate) fn new(reader: BufReader<R>, cols: Vec<Column>) -> Result<TsvReader<R>, Error>  {
        let mut lines = reader.lines();
        let header = lines.next().ok_or(Error::from("No header line"))??;
        let col_map = get_col_map(&header, &cols)?;
        Ok(TsvReader { lines, col_map, cols })
    }
}

impl<R: Read> Iterator for TsvReader<R> {
    type Item = Result<Row, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line_result| {
            line_result.map_err(Error::from).and_then(|line| {
                let mut row = Row::new();
                for (i, field) in line.split('\t').enumerate() {
                    if let Some(j) = self.col_map[i] {
                        let col = &self.cols[j];
                        row.add_value(&col.key, col.tpe.parse(field)?);
                    }
                }
                Ok(row)
            })
        })
    }
}
fn get_col_map(header: &str, cols: &[Column]) -> Result<Vec<Option<usize>>, Error> {
    let mut cols_found: Vec<bool> = vec![false; cols.len()];
    let mut col_map: Vec<Option<usize>> = Vec::new();
    for header_field in header.split('\t') {
        let mut found = false;
        for (j, col) in cols.iter().enumerate() {
            if col.key == header_field {
                if cols_found[j] {
                    return Err(Error::from(format!("Duplicate column: '{}'", header_field)));
                }
                cols_found[j] = true;
                col_map.push(Some(j));
                found = true;
                break;
            }
        }
        if !found {
            col_map.push(None);
        }
    }
    cols_found.iter().enumerate().find(|&(_, found)| !found)
        .map(|(j, _)|
            Err(Error::from(format!("Missing column: '{}'", cols[j].key)))
        )
        .unwrap_or(Ok(col_map))
}

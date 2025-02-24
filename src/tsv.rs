use std::io::{BufRead, BufReader, Read};
use std::marker::PhantomData;
use crate::error::Error;

pub(crate) trait TsvEater {
    type Row;
    fn new() -> Self;
    fn field(&mut self, name: &str, value: &str) -> Result<(), Error>;
    fn finish(&mut self) -> Result<Self::Row, Error>;
}

pub(crate) struct TsvReader<R: Read, E: TsvEater> {
    columns: Vec<String>,
    lines: std::io::Lines<BufReader<R>>,
    tsv_eater_phantom: PhantomData<E>,
}

impl<R: Read, E: TsvEater> TsvReader<R, E> {
    pub(crate) fn new(reader: BufReader<R>) -> Result<Self, Error> {
        let mut lines = reader.lines();
        let columns =
            lines.next().ok_or_else(|| Error::from("Empty TSV file"))??
                .split('\t').map(|s| s.to_string()).collect();
        Ok(TsvReader { columns, lines, tsv_eater_phantom: PhantomData })
    }
}

impl<R: Read, E: TsvEater> Iterator for TsvReader<R, E> {
    type Item = Result<E::Row, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            line.map_err(Error::from).and_then(|line| {
                let mut eater = E::new();
                for (name, value) in self.columns.iter().zip(line.split('\t')) {
                    eater.field(name, value)?;
                }
                eater.finish()
            })
        })
    }
}


use crate::error::Error;
use std::io::{BufRead, BufReader, Read};

pub(crate) trait TsvEater {
    type Row;
    fn field(&mut self, name: &str, value: &str) -> Result<(), Error>;
    fn finish(self) -> Result<Self::Row, Error>;
}

pub(crate) trait TsvEaterMaker {
    type Row;
    type Eater: TsvEater<Row = Self::Row>;
    fn make(&self) -> Self::Eater;
}
pub(crate) struct TsvReader<R: Read, M: TsvEaterMaker> {
    columns: Vec<String>,
    lines: std::io::Lines<BufReader<R>>,
    tsv_eater_maker: M
}

impl<R: Read, M: TsvEaterMaker> TsvReader<R, M> {
    pub(crate) fn new(reader: BufReader<R>, tsv_eater_maker: M) -> Result<Self, Error> {
        let mut lines = reader.lines();
        let columns =
            lines.next().ok_or_else(|| Error::from("Empty TSV file"))??
                .split('\t').map(|s| s.to_string()).collect();
        Ok(TsvReader { columns, lines, tsv_eater_maker })
    }
}

impl<R: Read, M: TsvEaterMaker> Iterator for TsvReader<R, M> {
    type Item = Result<M::Row, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            line.map_err(Error::from).and_then(|line| {
                let mut eater = self.tsv_eater_maker.make();
                for (name, value) in self.columns.iter().zip(line.split('\t')) {
                    eater.field(name, value)?;
                }
                eater.finish()
            })
        })
    }
}


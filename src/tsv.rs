use crate::error::Error;
use std::io::{BufRead, BufReader, Read};
use crate::s3::LineConsumer;

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
    separator: char,
    columns: Vec<String>,
    lines: std::io::Lines<BufReader<R>>,
    tsv_eater_maker: M
}

impl<R: Read, M: TsvEaterMaker> TsvReader<R, M> {
    pub(crate) fn new(reader: BufReader<R>, separator: char, tsv_eater_maker: M) -> Result<Self, Error> {
        let mut lines = reader.lines();
        let columns =
            lines.next().ok_or_else(|| Error::from("Empty TSV file"))??
                .split(separator).map(|s| s.to_string()).collect();
        Ok(TsvReader { separator, columns, lines, tsv_eater_maker })
    }
}

impl<R: Read, M: TsvEaterMaker> Iterator for TsvReader<R, M> {
    type Item = Result<M::Row, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            line.map_err(Error::from).and_then(|line| {
                parse_record(&self.tsv_eater_maker, &self.columns, line, self.separator)
            })
        })
    }
}

fn parse_record<M: TsvEaterMaker>(tsv_eater_maker: &M, columns: &[String], line: String, 
                                  separator: char) -> Result<M::Row, Error> {
    let mut eater = tsv_eater_maker.make();
    for (name, value) in columns.iter().zip(line.split(separator)) {
        eater.field(name, value)?;
    }
    eater.finish()
}

pub(crate) struct TsvConsumer<M: TsvEaterMaker, F: FnMut(M::Row) -> Result<(), Error>> {
    separator: char,
    columns: Option<Vec<String>>,
    tsv_eater_maker: M,
    consumer: F
}

impl<M: TsvEaterMaker, F: FnMut(M::Row) -> Result<(), Error>> TsvConsumer<M, F> {
    pub(crate) fn new(separator: char, tsv_eater_maker: M, consumer: F) -> Self {
        TsvConsumer { separator, columns: None, tsv_eater_maker, consumer }
    }
}

impl<M: TsvEaterMaker, F: FnMut(M::Row) -> Result<(), Error>> LineConsumer for TsvConsumer<M, F> {
    fn consume(&mut self, line: String) -> Result<(), Error> {
        if let Some(columns) = &self.columns {
            let item = 
                parse_record(&self.tsv_eater_maker, columns, line, self.separator)?;
            (self.consumer)(item)?;
        } else {
            self.columns = Some(line.split(self.separator).map(|s| s.to_string()).collect());
        }
        Ok(())
    }
}
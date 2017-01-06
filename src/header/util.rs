#![allow(missing_docs)]

use std::fmt::Display;
use std::io::{self, Write};
use std::str::{self, FromStr};

pub fn parse_value<T: FromStr>(s: &[Vec<u8>]) -> Result<T, ()> {
    if s.len() != 1 {
        return Err(());
    }
    str::from_utf8(s[0].as_slice())
        .ok()
        .and_then(|x| x.parse().ok())
        .ok_or(())
}

pub fn serialize_value<I, W, T>(mut iter: I, v: T) -> Result<(), io::Error>
    where I: Iterator<Item = W>,
          W: Write,
          T: Display
{
    write!(iter.next().unwrap(), "{}", v)
}

struct IterListHeader<'a> {
    values: &'a [Vec<u8>],
    line: usize,
    column: usize,
}

impl<'a> IterListHeader<'a> {
    fn new(values: &[Vec<u8>]) -> IterListHeader {
        IterListHeader {
            values: values,
            line: 0,
            column: 0,
        }
    }
}

impl<'a> Iterator for IterListHeader<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<&'a [u8]> {
        for line in self.line..self.values.len() {
            let value = &self.values[line];
            let mut maybe_start_column = None;
            let mut end_column = 0;
            for (column, _) in value.iter().enumerate().skip(self.column) {
                let byte = value[column];
                if byte != b' ' && byte != b'\t' && byte != b',' {
                    end_column = column + 1;
                    if maybe_start_column.is_none() {
                        maybe_start_column = Some(column)
                    }
                } else if byte == b',' {
                    if let Some(start_column) = maybe_start_column {
                        self.column = column + 1;
                        return Some(&value[start_column..end_column]);
                    }
                    maybe_start_column = None;
                }
            }
            self.line = line + 1;
            self.column = 0;
            if let Some(start_column) = maybe_start_column {
                return Some(&value[start_column..end_column]);
            }
        }
        None
    }
}

pub fn parse_list0<T: FromStr>(s: &[Vec<u8>]) -> Result<Vec<T>, ()> {
    let iter = IterListHeader::new(s);
    let items: Option<Vec<T>> = iter.map(|x| {
            str::from_utf8(x)
                .ok()
                .and_then(|x| x.parse().ok())
        })
        .collect();
    items.ok_or(())
}

pub fn parse_list1<T: FromStr>(s: &[Vec<u8>]) -> Result<Vec<T>, ()> {
    let list = try!(parse_list0(s));
    if list.is_empty() {
        return Err(());
    }
    Ok(list)
}

pub fn serialize_list<I, W, T>(mut iter: I, values: &[T]) -> Result<(), io::Error>
    where I: Iterator<Item = W>,
          W: Write,
          T: Display
{
    let mut w = iter.next().unwrap();
    for (i, v) in values.iter().enumerate() {
        if i != 0 {
            w.write_all(b", ")?;
        }
        write!(w, "{}", v)?;
    }
    Ok(())
}

pub fn parse_star(s: &[Vec<u8>]) -> Result<(), ()> {
    if s.len() != 1 {
        return Err(());
    }
    let mut star = false;
    for x in &s[0] {
        if *x == b' ' || *x == b'\t' {
            continue;
        } else if *x == b'*' {
            if star {
                return Err(());
            }
            star = true;
        }
    }
    Ok(())
}

pub fn parse_list1_star<T: FromStr>(s: &[Vec<u8>]) -> Result<Vec<T>, ()> {
    parse_star(s).map(|()| Vec::new()).or_else(|()| parse_list1(s))
}

pub fn serialize_list_star<I, W, T>(mut iter: I, values: &[T]) -> Result<(), io::Error>
    where I: Iterator<Item = W>,
          W: Write,
          T: Display
{
    if values.is_empty() {
        return iter.next().unwrap().write_all(b"*");
    }
    serialize_list(iter, values)
}

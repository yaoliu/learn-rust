use clap::Parser;
use colored::*;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Stdout, Write},
    ops::Range,
    path::Path,
};
use std::fs::read;

mod error;

pub use error::GrepError;

pub type StrategyFn<W, R> = fn(&Path, BufReader<R>, &Regex, &mut W) -> Result<(), GrepError>;


#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "YaoYao <liuyao@163.com>")]
pub struct GrepConfig {
    /// 用于查找的正则表达式
    pattern: String,
    /// 文件通配符
    glob: String,
}

impl GrepConfig {
    pub fn match_with_default(&mut self) -> Result<(), GrepError> {
        self.match_with(default_strategy)
    }

    pub fn match_with(&self, strategy: StrategyFn<Stdout, File>) -> Result<(), GrepError> {
        let regex = Regex::new(&self.pattern)?;
        // 返回一个列表 每个元素为一个文件名
        let files: Vec<_> = glob::glob(&self.glob)?.collect();
        files.into_par_iter().for_each(|v| {
            if let Ok(filename) = v {
                // 打开文件
                if let Ok(file) = File::open(&filename) {
                    let reader = BufReader::new(file);
                    let mut stdout = io::stdout();
                    if let Err(e) = strategy(filename.as_path(), reader, &regex, &mut stdout) {
                        println!("Internal error: {:?}", e);
                    }
                }
            };
        });
        Ok(())
    }
}

pub fn default_strategy<W: Write, R: Read>(
    path: &Path,
    reader: BufReader<R>,
    pattern: &Regex,
    writer: &mut W)
    -> Result<(), GrepError> {
    let matches: String = reader
        .lines()
        // 迭代器
        .enumerate()
        .map(|(lineno, line)| {
            line.ok().map(|line| {
                pattern.find(&line).map(|m| format_line(&line, lineno + 1, m.range()))
            }).flatten()
        }).filter_map(|v| v.ok_or(()).ok()).join("\\n");

    if !matches.is_empty() {
        writer.write(path.display().to_string().green().as_bytes())?;
        writer.write(b"\\n")?;
        writer.write(matches.as_bytes())?;
        writer.write(b"\\n")?;
    }
    Ok(())
}


pub fn format_line(line: &str, lineno: usize, range: Range<usize>) -> String {
    let Range { start, end } = range;
    let prefix = &line[..start];
    format!(
        "{0: >6}:{1: <3} {2}{3}{4}",
        lineno.to_string().blue(),
        (prefix.chars().count() + 1).to_string().cyan(),
        prefix,
        &line[start..end].red(),
        &line[end..]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn println_glob() {
        let files: Vec<_> = glob::glob("./src/*.rs").unwrap().collect();
        files.into_par_iter().for_each(|v| {
            if let Ok(filename) = v {
                println!("{:?}", filename.as_path())
            }
        })
    }

    #[test]
    fn format_line_should_work() {
        let result = format_line("Hello, world!", 1000, 7..10);
        println!("{:?}", result)
    }

    #[test]
    fn println_reader_should_work() {
        let path = Path::new("src/lib.rs");
        let file = File::open(path);
        let reader = BufReader::new(file.unwrap());
        let matches = reader.lines().enumerate().map(|(no, line)| {
            line.ok().map(|v| println!("{:?}", v))
        });
        for _ in matches {}
    }

    #[test]
    fn test() {
        // let a = [1, 2, 3];
        //
        // let mut iter = a.iter().map(|x| println!("{:?}", x));
        // iter.flatten();
        ;
        for x in (0..5).map(|x| x * 3) {
            println!("{}", x);
        }
    }
}
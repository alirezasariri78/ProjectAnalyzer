use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use serde::Serialize;

#[derive(Serialize)]
pub struct AnalyzeResultItem {
    postfix: String,
    files: usize,
    lines: usize,
    empty_lines: usize,
}

impl AnalyzeResultItem {
    pub fn new(postfix: String, files: usize, lines: usize, empty_lines: usize) -> Self {
        Self {
            postfix,
            files,
            lines,
            empty_lines,
        }
    }

    pub fn lines(&self) -> usize {
        self.lines
    }
    pub fn files(&self) -> usize {
        self.files
    }
    pub fn postfix(&self) -> &str {
        &self.postfix
    }
    pub fn empty_lines(&self) -> usize {
        self.empty_lines
    }
    pub fn total_lines(&self) -> usize {
        self.empty_lines + self.lines
    }
}

#[derive(Serialize)]
pub struct AnalyzeResult(Vec<AnalyzeResultItem>);

impl AnalyzeResult {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add(&mut self, postfix: &str, /*  content: Vec<u8>*/ content: BufReader<File>) {
        let position = self.0.iter().position(|x| x.postfix == postfix);

        let mut lines = 0; //content.iter().filter(|i| **i == b'\n').count();
        let mut empty_lines = 0; //content.iter().filter(|lines| **lines).count();
        for line in content.lines() {
            if line.unwrap().trim().is_empty() {
                empty_lines += 1;
            } else {
                lines += 1;
            }
        }

        match position {
            Some(position) => {
                self.0[position].files += 1;
                self.0[position].lines += lines;
            }
            None => self.0.push(AnalyzeResultItem::new(
                postfix.to_string(),
                1,
                lines,
                empty_lines,
            )),
        }
    }

    pub fn iter(&self) -> &Vec<AnalyzeResultItem> {
        &self.0
    }
}

impl ToString for AnalyzeResult {
    fn to_string(&self) -> String {
        let mut table = crate::ui::table::Table::new();

        table.write_center("ProjectAnalyzer");

        table.empty_line();
        table.write("https://github.com/ali77gh/ProjectAnalyzer");
        table.empty_line();

        for item in self.iter() {
            if item.lines == 0 {
                continue;
            }
            table.draw_line();
            table.write(format!("{} files result:", item.postfix()));
            table.write(format!("  files: {}", item.files()));
            table.write(format!("  lines: {} ", item.lines()));
            table.write(format!("  empty lines: {} ", item.empty_lines()));
            table.write(format!("  total lines: {} ", item.total_lines()));
        }
        table.render_table()
    }
}

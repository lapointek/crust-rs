use clap::Parser;
use owo_colors::OwoColorize;
use std::{
    fs,
    path::{Path, PathBuf},
};
use strum::Display;
use tabled::settings::object::Columns;
use tabled::settings::object::Rows;
use tabled::{
    Table, Tabled,
    settings::{Color, Style},
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = "ls command built in Rust")]
struct Cli {
    path: Option<PathBuf>,
}

#[derive(Debug, Display)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled)]
struct FileEntry {
    name: String,
    e_type: EntryType,
    len_bytes: u64,
    modified: String,
}

fn main() {
    let cli = Cli::parse();

    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            let get_files = get_files(&path);
            let mut table = Table::new(get_files);
            table.with(Style::rounded());
            table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
            table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
            table.modify(Columns::one(3), Color::FG_BRIGHT_MAGENTA);
            table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
            println!("{}", table);
        } else {
            println!("{}", "Path does not exist".red());
        }
    } else {
        println!("{}", "error reading directory".red());
    }
}

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                map_data(file, &mut data);
            }
        }
    }
    data
}

fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(&file.path()) {
        data.push(FileEntry {
            name: file
                .file_name()
                .into_string()
                .unwrap_or("unknown name".into()),
            e_type: if meta.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            },
            len_bytes: meta.len(),
            modified: "".to_string(),
        })
    }
}

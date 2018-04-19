extern crate clap;
extern crate colored;
extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{Arg, App, SubCommand};
use colored::*;
use regex::Regex;


#[derive(Debug)]
struct Task {
    title: String,
    tag: String
}


#[derive(Debug)]
struct WorkDay {
    date: String,
    tasks: Vec<Task>,
}


fn parse_file(file: File) -> Vec<WorkDay>{
    let reader = BufReader::new(&file);
    let mut work: Vec<WorkDay> = Vec::new();
    let mut day = WorkDay {
        date: String::from(""),
        tasks: vec![],
    };
    let date_pattern = Regex::new(
        r"= (?P<date>\d{4}-\d{2}-\d{2}) \w+ =").unwrap();
    let task_pattern = Regex::new(
        r"\* \[.] (?P<title>.*) (?P<tag>:\w+:?\w+:?)").unwrap();
    for line in reader.lines() {
        let l = line.unwrap();
        if l.starts_with("= 20") {
            work.push(day);
            let d = date_pattern.captures(&l).unwrap();
            day = WorkDay {
                date: d["date"].to_string(),
                tasks: Vec::new(),
            };
        } else if l.starts_with("= TODO") {
            break;
        } else if l.starts_with("* [") {
            match task_pattern.captures(&l) {
                None => {},
                Some(t) => {
                    day.tasks.push(Task {
                        title: t["title"].to_string(),
                        tag: t["tag"].to_string(),
                    });
                },
            }
        }
    }
    work.push(day);
    work
}


fn report_scrum(work: Vec<WorkDay>, filters: Vec<&str>) {
    let len = work.len() - 1;

    println!("{}", "Yesterday:".red().bold());
    for task in work.get(len - 1).unwrap().tasks.iter() {
        let mut skip = if filters.len() > 0 { true } else { false };
        for filter in filters.clone() {
            if task.tag.replace(":", "").starts_with(filter) {
                skip = false;
                break;
            }
        }
        if skip {
            continue;
        }
        println!("{}", task.title);
    }
    println!("{}", "Today:".red().bold());
    for task in work.get(len).unwrap().tasks.iter() {
        let mut skip = if filters.len() > 0 { true } else { false };
        for filter in filters.clone() {
            if task.tag.replace(":", "").starts_with(filter) {
                skip = false;
                break;
            }
        }
        if skip {
            continue;
        }
        println!("{}", task.title);
    }
}


fn main() {
    let parser = App::new("Worktrack")
                         .arg(Arg::with_name("file")
                              .help("File path to worktrack")
                              .required(true))
                         .subcommand(SubCommand::with_name("report")
                                     .about("Report worktrack")
                                     .arg(Arg::with_name("name")
                                          .help("Report type")
                                          .possible_values(&["scrum", ])
                                          .required(true))
                                     .arg(Arg::with_name("filter")
                                          .help("Filter by tag")
                                          .short("f")
                                          .long("filter")
                                          .multiple(true)
                                          .takes_value(true)))
                         .get_matches();

    let filepath = File::open(parser.value_of("file").unwrap()).unwrap();
    let work = parse_file(filepath);

    if let Some(report) = parser.subcommand_matches("report") {

        // Parse all tag filters
        let filters = match report.values_of("filter") {
            Some(values) => values.collect::<Vec<&str>>(),
            _ => Vec::new(),
        };

        // println!("called with filters: {}", filters.join(", "));


        match report.value_of("name") {
            Some("scrum") => {
                report_scrum(work, filters);
            },
            Some(name) => {
                println!("Invalid report name: {}", name);
            },
            _ => {},
        }
    }

}

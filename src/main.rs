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


// Parse an entire file and return the list of WorkDays
fn parse_file(file: File) -> Vec<WorkDay>{
    // Date and task regex parsers
    let date_pattern = Regex::new(
        r"= (?P<date>\d{4}-\d{2}-\d{2}) \w+ =").unwrap();
    let task_pattern = Regex::new(
        r"\* \[.] (?P<title>.*) (?P<tag>:\w+:?\w+:?)").unwrap();

    let mut work: Vec<WorkDay> = Vec::new();
    let mut day = WorkDay {
        date: String::from(""),
        tasks: vec![],
    };

    // Parse the entire file
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        let l = line.unwrap();

        if date_pattern.is_match(&l) {
            work.push(day);
            let d = date_pattern.captures(&l).unwrap();
            day = WorkDay {
                date: d["date"].to_string(),
                tasks: Vec::new(),
            };
        } else if task_pattern.is_match(&l) {
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
    // Add the last parsed work day and return the parsed file
    work.push(day);
    work
}


// Report the last two days of the worktrack file
fn report_scrum(work: Vec<WorkDay>, filters: Vec<&str>) {
    let len = work.len() - 1;

    println!("{}", "Yesterday:".red().bold());
    print_scrum_tasks(work.get(len - 1).unwrap(), &filters);

    println!("{}", "Today:".red().bold());
    print_scrum_tasks(work.get(len).unwrap(), &filters);
}


// Print the scrum tasks from a WorkDay
fn print_scrum_tasks(workday: &WorkDay, filters: &Vec<&str>) {
    for task in workday.tasks.iter() {
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
                         .subcommand(SubCommand::with_name("scrum")
                                     .about("Report scrum")
                                     .arg(Arg::with_name("filter")
                                          .help("Filter by tag")
                                          .short("f")
                                          .long("filter")
                                          .multiple(true)
                                          .takes_value(true)))
                         .get_matches();

    let filepath = File::open(parser.value_of("file").unwrap()).unwrap();

    let work = parse_file(filepath);

    if let Some(scrum) = parser.subcommand_matches("scrum") {

        // Parse all tag filters
        let filters = match scrum.values_of("filter") {
            Some(values) => values.collect::<Vec<&str>>(),
            _ => Vec::new(),
        };
        report_scrum(work, filters);
    }

}

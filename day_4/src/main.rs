#![feature(duration_as_u128)]
extern crate lazy_static;
extern crate regex;
extern crate simple_error;

use lazy_static::lazy_static;
use regex::Regex;
use simple_error::bail;
use std::error::Error;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;
use std::collections::HashMap;
use std::time::Instant;
/**
 * Given a bunch of times like:
 * [1518-11-01 00:00] Guard #10 begins shift
 *  [1518-11-01 00:05] falls asleep
 *  [1518-11-01 00:25] wakes up
 *
 * Find the gaurd that sleeps the most
 * Find the minute he is most often asleep
 */
fn main() {
    let start = Instant::now();
    solve_a("input.txt");
    let end = Instant::now();
    println!("Finished in : {} ms", end.duration_since(start).as_millis());
}

fn solve_a(filename: &str) -> u64{
    let entries = parse_all_entries(filename);
    let gs = parse_guards(&entries);
    let (gid, _) = most_sleep(&gs);
    let (most_common, minutes) = most_common_minute(&gs[&gid]);
    most_common * gid
}

/// minute, slept in that minute
fn most_common_minute(g: &Gaurd) -> (u64, u64) {
    let mut minutes: Vec<u64> = Vec::new();
    for _ in 0..60 {
        minutes.push(0);
    }

    for (start, end) in &g.sleeps {
        let st: u64 = start.minute;
        let ed: u64;
        if start.minute > end.expect("minute").minute {
            ed = end.unwrap().minute + 60;
        } else {
            ed = end.unwrap().minute;
        }

        for i in 0..ed - st {
            let idx = (i + st) % 60;
            minutes[idx as usize] += 1;
        }
    }

    let max = minutes.iter().enumerate().fold((0, 0), |(im, iv), (i, v)| {
        if *v > iv {
            (i as u64, *v)
        } else {
            (im as u64, iv)
        }
    });
    max
}

fn most_sleep(gs: &HashMap<u64, Gaurd>) -> (u64, u64) {
    let mut most_time = 0; 
    let mut id = 0;
    for (gid, g) in gs.iter() {
        let slept: u64 = g.sleeps.iter().map(|(start, end)| {
            end.expect("no end").since(start)
        }).sum();
        if slept > most_time {
            most_time = slept;
            id = *gid;
        }
    }
    (id, most_time)
}

fn parse_guards(entries: &[Entry]) -> HashMap<u64, Gaurd> {
    let mut gmap : HashMap<u64, Gaurd> = HashMap::new();
    let mut current_guard: u64 = 0;
    gmap.insert(0, Gaurd {id: 0, sleeps: Vec::new()});
    for entry in entries {
        match entry {
            Entry::begin(id) => {
                gmap.entry(*id).or_insert(Gaurd {id: *id, sleeps: Vec::new()});
                current_guard = *id;
            },
            Entry::sleep(t) => {
                gmap.get_mut(&current_guard).expect(&format!("no gaurd {}", current_guard)).sleeps.push((*t, None))
            },
            Entry::wake(t) => {
                let (_, wake)= gmap.get_mut(&current_guard).expect("no gaurd").sleeps.last_mut().expect("no last");
                *wake = Some(*t);
            }
        }
    }
    return gmap;
}

fn parse_all_entries(filename: &str) -> Vec<Entry> {
    let f = std::fs::File::open(filename).unwrap();
    let r = BufReader::new(f);
    r.lines()
        .map(|line| line.unwrap().parse::<Entry>().unwrap())
        .collect()
}

#[derive(Debug, PartialEq)]
enum Entry {
    begin(u64), // id
    sleep(TimeEntry),
    wake(TimeEntry),
}

impl FromStr for Entry {
    type Err = Box<dyn Error>;

    fn from_str(entry: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref begin: Regex = Regex::new(
                r"Guard #(?P<ID>\d*) begins shift"
            )
            .expect("failed regex");
            static ref wake: Regex = Regex::new(
                r"wakes up"
            )
            .expect("failed regex");
            static ref sleep: Regex = Regex::new(
                r"falls asleep"
            )
            .expect("failed regex");
        };

        let time_end = 18;
        let time_st = &entry[..time_end];
        let time = time_st.parse::<TimeEntry>();
        let entry = &entry[time_end + 1..];

        if begin.is_match(entry) {
            let id = begin.captures(entry).unwrap().name("ID").unwrap().as_str().parse()?;
            return Ok(Entry::begin(id));
        } else if wake.is_match(entry) {
            return Ok(Entry::wake(time?));
        } else {
            return Ok(Entry::sleep(time?));
        }


    }
}

#[derive(Debug)]
struct Gaurd {
    id: u64,
    sleeps: Vec<(TimeEntry, Option<TimeEntry>)>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct TimeEntry {
    year: u64,
    month: u64,
    day: u64,
    hour: u64,
    minute: u64,
}

impl FromStr for TimeEntry {
    type Err = Box<dyn Error>;

    fn from_str(entry: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref reg: Regex = Regex::new(
                r"\[(?P<year>\d*)\-(?P<month>\d*)\-(?P<day>\d*) (?P<hour>\d*):(?P<minute>\d*)\]"
            )
            .expect("bad regex");
        }
        let caps = match reg.captures(entry) {
            Some(caps) => caps,
            _ => bail!("failed to parse entry"),
        };
        Ok(TimeEntry {
            year: caps.name("year").unwrap().as_str().parse()?,
            month: caps.name("month").unwrap().as_str().parse()?,
            day: caps.name("day").unwrap().as_str().parse()?,
            hour: caps.name("hour").unwrap().as_str().parse()?,
            minute: caps.name("minute").unwrap().as_str().parse()?,
        })
    }
}

impl TimeEntry {
    fn val(self) -> u64 {
        return self.minute + self.hour * 60 + self.day * 24 * 60 + self.month * 31 * 24 * 60 + self.year * 365 * 31 * 24 * 60;
    }
    fn since(self, other: &TimeEntry) -> u64 {
        self.val() - other.val()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let st = "[1518-11-01 00:00]";
        let entry = st.parse::<TimeEntry>().unwrap();
        assert_eq!(
            entry,
            TimeEntry {
                year: 1518,
                month: 11,
                day: 1,
                hour: 0,
                minute: 0
            }
        );
    }

    #[test]
    fn test_parse_entry () {
        let st = "[1518-11-01 00:00] Guard #10 begins shift";
        let parsed = st.parse::<Entry>().unwrap();
        assert_eq!(parsed, Entry::begin(10));
    }

    #[test]
    fn test_input() {
        let res = solve_a("test.txt");
        assert_eq!(res, 240);
    }
}
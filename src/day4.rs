use std::fs::File;
use std::io::Read;
use regex::Regex;
use chrono::{DateTime, Utc, TimeZone, Timelike};
use std::collections::HashMap;

type GuardId = u32;
type GuardSleepFrequency = HashMap<GuardId, HashMap<u32, u32>>;

#[derive(Debug)]
struct LogEntry {
    timestamp: DateTime<Utc>,
    payload: String,
}

#[derive(Debug)]
struct GuardEventLog {
    guard_id: u32,
    events: Vec<GuardEvent>,
}

#[derive(Debug)]
enum GuardEvent {
    Asleep(u32),
    WokeUp(u32),
}

pub fn part_one() -> u32 {
    let guard_event_log = create_guard_event_log(assemble_log_entries());

    let frequency_list = build_frequency_list_from_event_log(&guard_event_log);

    let most_sleepy_guard = get_most_sleepy_guard(&frequency_list);
    let most_sleepy_minute = get_most_sleepy_minute_for_guard(most_sleepy_guard, &frequency_list);

    most_sleepy_guard * most_sleepy_minute
}

pub fn part_two() -> u32 {
    let frequency_list = build_frequency_list_from_event_log(
        &create_guard_event_log(
            assemble_log_entries()
        )
    );

    let (guard_id, sleepiest_minute) = get_most_sleepy_guard_for_minute(&frequency_list);

    guard_id * sleepiest_minute
}

fn assemble_log_entries() -> Vec<LogEntry> {
    let mut file = File::open("src/inputs/day4.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    lazy_static!(
        static ref RE: Regex = Regex::new(r"(?x)
            \[
            (?P<timestamp>\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2})
            \]
            \s+
            (?P<entry>.*)")
            .unwrap();
    );

    let mut logs = input
        .lines()
        .filter_map(|line| RE.captures(line))
        .map(|captures| LogEntry {
            timestamp: Utc.datetime_from_str(&captures["timestamp"], "%Y-%m-%d %H:%M").unwrap(),
            payload: captures["entry"].to_string(),
        })
        .collect::<Vec<LogEntry>>();

    logs.sort_by(|lhs, rhs| lhs.timestamp.cmp(&rhs.timestamp));

    logs
}

fn create_guard_event_log(logs: Vec<LogEntry>) -> HashMap<GuardId, Vec<GuardEvent>> {
    lazy_static!(
        static ref RE: Regex = Regex::new(r"(?x)Guard\s\#(?P<guard_id>\d+)|(?P<sleep>.+)").unwrap();
    );

    let mut guard_event_log: Vec<GuardEventLog> = Vec::new();
    for i in 0..logs.len() {
        if let Some(caps) = RE.captures(&logs[i].payload) {
            if let Some(guard_id) = caps.name("guard_id") {
                let mut guard_events = GuardEventLog {
                    guard_id: guard_id.as_str().parse().unwrap(),
                    events: Vec::new(),
                };

                for j in i + 1..logs.len() {
                    if let Some(caps) = RE.captures(&logs[j].payload) {
                        if let Some(_) = caps.name("guard_id") {
                            break;
                        }

                        match &caps["sleep"] {
                            "falls asleep" => guard_events.events.push(GuardEvent::Asleep(logs[j].timestamp.minute())),
                            "wakes up" => guard_events.events.push(GuardEvent::WokeUp(logs[j].timestamp.minute())),
                            _ => break,
                        };
                    }
                }
                guard_event_log.push(guard_events);
            }
        }

        continue;
    }

    guard_event_log.sort_by(|lhs, rhs| lhs.guard_id.cmp(&rhs.guard_id));
    let mut coalesced_log = HashMap::new();
    for mut log in guard_event_log {
        coalesced_log
            .entry(log.guard_id)
            .and_modify(|list: &mut Vec<GuardEvent>| list.append(&mut log.events))
            .or_insert(Vec::new());
    }

    coalesced_log
}

fn build_frequency_list_from_event_log(event_log: &HashMap<GuardId, Vec<GuardEvent>>) -> GuardSleepFrequency {
    let mut frequency_list = GuardSleepFrequency::new();

    for log in event_log {
        if log.1.len() < 1 {
            continue;
        }

        let mut i = 0;
        let mut sleep_freq = HashMap::new();

        while i < log.1.len() {
            let start = match log.1[i] {
                GuardEvent::Asleep(i) => i,
                GuardEvent::WokeUp(i) => i,
            };

            let end = match log.1[i + 1] {
                GuardEvent::Asleep(i) => i,
                GuardEvent::WokeUp(i) => i,
            };

            for minute in start..end {
                *sleep_freq
                    .entry(minute)
                    .or_default() += 1;
            }

            i += 2;
        }

        frequency_list.insert(*log.0, sleep_freq);
    }

    frequency_list
}

fn get_most_sleepy_guard(frequency_list: &GuardSleepFrequency) -> GuardId {
    let mut sleep_totals: HashMap<GuardId, u32> = HashMap::new();

    for guard in frequency_list {
        let sleep_total = guard.1.values().sum();
        sleep_totals.insert(*guard.0, sleep_total);
    }

    *sleep_totals
        .iter()
        .max_by(|lhs, rhs| lhs.1.cmp(rhs.1))
        .unwrap().0
}

fn get_most_sleepy_minute_for_guard(guard_id: GuardId, frequency_list: &GuardSleepFrequency) -> u32 {
    *frequency_list
        .get(&guard_id)
        .unwrap()
        .iter()
        .max_by(|lhs, rhs| lhs.1.cmp(rhs.1))
        .unwrap()
        .0
}

fn get_most_sleepy_guard_for_minute(frequency_list: &GuardSleepFrequency) -> (GuardId, u32) {
    let mut sleepiest_minute_map: HashMap<GuardId, u32> = HashMap::new();

    for entry in frequency_list {
        let sleepiest_minute = *entry.1
            .iter()
            .max_by(|(_, &lhs_value), (_, rhs_value)| lhs_value.cmp(rhs_value))
            .unwrap()
            .0;
        sleepiest_minute_map.insert(*entry.0, sleepiest_minute);
    }

    let (&guard_id, &sleepiest_minute) = sleepiest_minute_map
        .iter()
        .max_by(|(_, &lhs_sleepiest_minute), (_, rhs_sleepiest_minute)| lhs_sleepiest_minute.cmp(rhs_sleepiest_minute))
        .unwrap();

    (guard_id, sleepiest_minute)
}

use chrono::Duration;
use chrono::NaiveDate;
use chrono::NaiveTime;
use std::collections::HashMap;

#[derive(Debug)]
pub enum DelayType {
    Minus,      // affects all occurrences
    MinusMinus, // affects only the first scheduled occurrence
}

#[derive(Debug)]
pub enum RepeaterType {
    Plus,     // add period to time, displayed as "+"
    PlusPlus, // keep adding period to time until time reaches future, displayed as "++"
    DotPlus,  // add period to current time, displayed as ".+"
}

#[derive(Debug)]
pub struct Repeater {
    repeat: Duration,
    periodicity_type: RepeaterType,
}

#[derive(Debug)]
pub struct Delay {
    delay: Duration,
    delay_type: DelayType,
}

#[derive(Debug)]
pub struct Timestamp {
    date: NaiveDate,
    start_time: Option<NaiveTime>,
    end_time: Option<NaiveTime>,
    repeater: Option<Repeater>,
    delay: Option<Delay>,
}

#[derive(Debug)]
pub struct Task {
    title: String,
    body: String,
    children: Vec<Task>,
    priority: Option<u32>,
    keyword: String,
    tags: Vec<String>,
    scheduled: Option<Timestamp>,
    deadline: Option<Timestamp>,
    created: Option<Timestamp>,
    properties: HashMap<String, String>,
}

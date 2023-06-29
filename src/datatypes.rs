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
    kind: RepeaterType,
}

#[derive(Debug)]
pub struct Delay {
    delay: Duration,
    kind: DelayType,
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
enum KeywordType {
    Active, // considered as "todo"
    Incactive // Considered as "done"
}

#[derive(Debug)]
pub struct Keyword {
    name: String,
    kind: KeywordType
}

#[derive(Debug)]
pub struct Task {
    title: String,
    body: String,
    children: Vec<Task>,
    priority: Option<u32>,
    keyword: Option<Keyword>,
    tags: Vec<String>,
    scheduled: Option<Timestamp>,
    deadline: Option<Timestamp>,
    created: Option<Timestamp>,
    properties: HashMap<String, String>,
}

#[derive(Debug)]
pub struct NotebookMetadata {
    title: String,
    keywords: Vec<Keyword>,
}

#[derive(Debug)]
pub struct Notebook{
    metadata: NotebookMetadata,
    tasks: Vec<Task>,
}

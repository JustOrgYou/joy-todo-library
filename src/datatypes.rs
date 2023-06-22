use chrono::Duration;
use chrono::NaiveDate;
use chrono::NaiveTime;
use std::collections::HashMap;
use std::sync::Arc;

pub type SharedString = Arc<str>;
pub type SharedInt = Arc<u32>;
pub type SharedPeriodicDateTime = Arc<Timestamp>;
pub type SharedSlice<T> = Arc<[T]>;

/// A word is a non-empty sequence of non-whitespace characters.
/// Here are a few test cases:
/// ```
/// use joy_todo_library::datatypes::SharedWord;
///
/// assert!(SharedWord::new("word".into()).is_some());
/// assert!(SharedWord::new("also_word".into()).is_some());
/// assert!(SharedWord::new("not word".into()).is_none());
/// assert!(SharedWord::new("".into()).is_none());
/// ```
#[derive(Debug, PartialEq)]
pub struct SharedWord(SharedString);

impl SharedWord {
    pub fn new(word: SharedString) -> Option<Self> {
        if word.len() == 0 || word.chars().any(|c| c.is_whitespace()) {
            None
        } else {
            Some(SharedWord(word))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A line is a sequence of characters that does not contain a newline.
/// Here are a few test cases:
/// ```
/// use joy_todo_library::datatypes::SharedLine;
///
///assert!(SharedLine::new("".into()).is_some());
///assert!(SharedLine::new("line".into()).is_some());
///assert!(SharedLine::new("line\nline".into()).is_none());
/// ```
#[derive(Debug, PartialEq)]
pub struct SharedLine(SharedString);

impl SharedLine {
    pub fn new(line: SharedString) -> Option<Self> {
        if line.chars().any(|c| c == '\n') {
            None
        } else {
            Some(SharedLine(line))
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

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
    title: SharedLine,
    body: SharedString,
    children: SharedSlice<Task>,
    priority: Option<SharedInt>,
    keyword: SharedWord,
    tags: SharedSlice<SharedWord>,
    scheduled: Option<SharedPeriodicDateTime>,
    deadline: Option<SharedPeriodicDateTime>,
    created: Option<SharedPeriodicDateTime>,
    properties: HashMap<SharedWord, SharedString>,
}

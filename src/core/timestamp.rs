use chrono::{Duration, NaiveDate, NaiveTime};

#[derive(Debug, PartialEq)]
pub enum DelayType {
    Minus,      // affects all occurrences
    MinusMinus, // affects only the first scheduled occurrence
}

#[derive(Debug, PartialEq)]
pub enum RepeaterType {
    Plus,     // add period to time, displayed as "+"
    PlusPlus, // keep adding period to time until time reaches future, displayed as "++"
    DotPlus,  // add period to current time, displayed as ".+"
}

#[derive(Debug, PartialEq)]
pub struct Repeater {
    repeat: Duration,
    kind: RepeaterType,
}

#[derive(Debug, PartialEq)]
pub struct Delay {
    delay: Duration,
    kind: DelayType,
}

#[derive(Debug, PartialEq)]
pub struct Timestamp {
    date: NaiveDate,
    start_time: Option<NaiveTime>,
    end_time: Option<NaiveTime>,
    repeater: Option<Repeater>,
    delay: Option<Delay>,
}

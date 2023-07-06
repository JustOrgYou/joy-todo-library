use chrono::{Duration, NaiveDate, NaiveTime};
use derive_new::new;
use getset::Getters;
use typed_builder::TypedBuilder;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DelayType {
    Minus,      // affects all occurrences
    MinusMinus, // affects only the first scheduled occurrence
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RepeaterType {
    Plus,     // add period to time, displayed as "+"
    PlusPlus, // keep adding period to time until time reaches future, displayed as "++"
    DotPlus,  // add period to current time, displayed as ".+"
}

#[derive(new, Getters, Debug, PartialEq, Clone, Copy)]
pub struct Repeater {
    repeat: Duration,
    kind: RepeaterType,
}

#[derive(new, Getters, Debug, PartialEq, Clone, Copy)]
pub struct Delay {
    delay: Duration,
    kind: DelayType,
}

#[derive(TypedBuilder, Getters, Debug, PartialEq, Clone)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct Timestamp {
    #[builder(setter(!strip_option))]
    date: NaiveDate,
    start_time: Option<NaiveTime>,
    end_time: Option<NaiveTime>,
    repeater: Option<Repeater>,
    delay: Option<Delay>,
}

use super::timestamp::*;
use chrono::{DateTime, Utc};
use derive_new::new;
use getset::{Getters, Setters};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Debug, PartialEq, Clone)]
pub enum KeywordType {
    Active,    // considered as "todo"
    Incactive, // Considered as "done"
}

#[derive(new, Getters, Debug, PartialEq, Clone)]
pub struct Keyword {
    name: String,
    kind: KeywordType,
}

#[derive(new, Getters, Debug, PartialEq, Clone)]
pub struct ArchiveProperties {
    archive_time: DateTime<Utc>,
    archive_file: String,
    archive_category: String, // name of the Notebook in which the task was archived
    archive_todo: Keyword,    // keyword of the task when it was archived
}

#[derive(new, Getters, Debug, PartialEq, Clone)]
pub struct TaskSystemInfo {
    // id: String,       // TODO: do we need runtime id?
    line_number: u32, // line number of title in the file
}

#[derive(Getters, Setters, TypedBuilder, Debug, PartialEq, Clone)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct Task {
    #[builder(!default, setter(!strip_option))]
    system_info: TaskSystemInfo,
    /// If level is 0 it is considered as not specified (e.g. in org mode without any asterisk)
    #[builder(!default, setter(!strip_option))]
    pub level: u32,
    priority: Option<u32>,
    keyword: Option<Keyword>,
    #[builder(!default, setter(!strip_option))]
    title: String,
    #[builder(default, setter(!strip_option))]
    tags: Vec<String>,
    closed: Option<Timestamp>,
    scheduled: Option<Timestamp>,
    deadline: Option<Timestamp>,
    created: Option<DateTime<Utc>>,
    archive: Option<ArchiveProperties>,
    #[builder(default, setter(!strip_option))]
    properties: HashMap<String, String>,
    #[builder(!default, setter(!strip_option))]
    body: String,
}

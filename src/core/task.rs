use std::collections::HashMap;
use chrono::{DateTime, Utc};
use super::timestamp::*;

#[derive(Debug, PartialEq)]
pub enum KeywordType {
    Active,    // considered as "todo"
    Incactive, // Considered as "done"
}

#[derive(Debug, PartialEq)]
pub struct Keyword {
    pub name: String,
    pub kind: KeywordType,
}

#[derive(Debug, PartialEq)]
pub struct ArchiveProperties {
    pub archive_time: DateTime<Utc>,
    pub archive_file: String,
    pub archive_category: String,
    pub archive_todo: Keyword,
}

#[derive(Debug, PartialEq)]
pub struct TaskSystemInfo {
    pub uuid: String,     // runtime unique identifier
    pub line_number: u32, // line number of title in the file
}

#[derive(Debug, PartialEq)]
pub struct Task {
    pub system_info: TaskSystemInfo,
    pub title: String,
    pub body: String,
    pub children: Vec<Task>,
    pub priority: Option<u32>,
    pub keyword: Option<Keyword>,
    pub tags: Vec<String>,
    pub closed: Option<Timestamp>,
    pub scheduled: Option<Timestamp>,
    pub deadline: Option<Timestamp>,
    pub created: Option<DateTime<Utc>>,
    pub archive: Option<ArchiveProperties>,
    pub properties: HashMap<String, String>,
}

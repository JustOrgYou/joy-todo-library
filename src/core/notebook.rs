use super::task::*;

#[derive(Debug, PartialEq)]
pub struct NotebookMetadata {
    title: Option<String>, // title may be set inside a file
    archive_file: Option<String>, // archive file may be set inside a file
    keywords: Vec<Keyword>, // keywords may be set inside a file
}

#[derive(Debug, PartialEq)]
pub struct Notebook {
    pub metadata: NotebookMetadata,
    pub tasks: Vec<Task>,
}


impl Notebook {
    pub fn empty() -> Notebook {
        Notebook {
            metadata: NotebookMetadata {
                title: None,
                archive_file: None,
                keywords: Vec::new(),
            },
            tasks: Vec::new(),
        }
    }
    pub fn set_title(&mut self, title: String) {
        self.metadata.title = Some(title);
    }
    pub fn set_archive_file(&mut self, archive_file: String) {
        self.metadata.archive_file = Some(archive_file);
    }
    pub fn set_keywords(&mut self, keywords: Vec<Keyword>) {
        self.metadata.keywords = keywords;
    }
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}

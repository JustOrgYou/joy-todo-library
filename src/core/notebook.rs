use crate::core::task::{Task, Keyword};
use derive_new::new;
use getset::{Getters, Setters};

/// Metadata that may be set inside the file
#[derive(new, Debug, PartialEq, Default)]
pub struct NotebookMetadata {
    pub title: Option<String>,
    pub archive_file: Option<String>,
    pub keywords: Vec<Keyword>,
}

#[derive(new, Getters, Setters, Debug, PartialEq, Default)]
pub struct Notebook<'a> {
    #[getset(get = "pub", set = "pub")]
    metadata: NotebookMetadata,
    #[getset(get = "pub")]
    tasks: Vec<&'a mut Task>,
}

impl<'a> Notebook<'a> {
    fn add_task(&mut self, task: &'a mut Task) {
        let _ = &self.tasks.push(task);
    }
    fn find_task(&mut self) -> &Task {
        self.tasks.get(0).expect("TODO")
    }
    fn find_task_mut(&mut self) -> &mut Task {
        self.tasks.get_mut(0).expect("TODO")
    }
    // TODO: implement removing and searching operations
}

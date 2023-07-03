use std::collections::HashMap;
use crate::core::notebook::Notebook;
use crate::core::task::*;

pub fn decode(str: &str) -> Result<Notebook, String> {
    let task = Task{
        system_info: TaskSystemInfo{
            uuid: "123".to_string(),
            line_number: 1,
        },
        title: "Task Title".to_string(),
        body: "Some body told me the world is gonna roll me".to_string(),
        children: Vec::new(),
        priority: None,
        keyword: Some(
            Keyword{
                name: "TODO".to_string(),
                kind: KeywordType::Active,
            }
        ),
        tags: Vec::new(),
        closed: None,
        scheduled: None,
        deadline: None,
        created: None,
        archive: None,
        properties: HashMap::new(),
    };
    let mut notebook = Notebook::empty();
    notebook.add_task(task);
    Ok(Notebook::empty())
}

// test
#[test]
fn test_decode() {
    let n = decode("").unwrap();
    let x = &n.tasks[0];
    // assert_eq!(n.tasks[0], Task{
    //     system_info: TaskSystemInfo{
    //         uuid: "123".to_string(),
    //         line_number: 1,
    //     },
    //     title: "Task Title".to_string(),
    //     body: "Some body told me the world is gonna roll me".to_string(),
    //     children: Vec::new(),
    //     priority: None,
    //     keyword: Some(
    //         Keyword{
    //             name: "TODO".to_string(),
    //             kind: KeywordType::Active,
    //         }
    //     ),
    //     tags: Vec::new(),
    //     closed: None,
    //     scheduled: None,
    //     deadline: None,
    //     created: None,
    //     archive: None,
    //     properties: HashMap::new(),
    // });
}

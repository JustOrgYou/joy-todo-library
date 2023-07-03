use crate::core::notebook::Notebook;

pub fn encode(notebook: &Notebook) -> String {
        "\
#+TITLE: Title
#+TODO: TODO(t) | DONE(d)

* TODO Task Title
:PROPERTIES:
:CREATED:  [2023-07-03 Mon 13:14]
:END:
Some body told me the world is gonna roll me
".to_string()

}

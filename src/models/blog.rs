use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    slug: String,
    content: String,
    frontmatter: Option<FrontMatter>,
    hidden: bool,
}

fn slugify(file_name: &str) -> String {
    file_name.split_whitespace().collect::<Vec<_>>().join("-")
}

impl Post {
    pub fn from_file(file: &PathBuf) -> Self {
        let contents = fs::read_to_string(file).expect("Panics are okay...");
        Self::from_md(file, &contents)
    }

    pub fn from_md(file: &PathBuf, md: &str) -> Self {
        let mut lines = md.trim().lines();
        let mut frontmatter: Option<FrontMatter> = None;
        let content = if let Some(line) = lines.next() {
            if line.trim() == "---" {
                let mut line = lines.next();
                let mut json_str = String::new();
                while line.is_some() && line.unwrap().trim() != "---" {
                    json_str.push_str(line.unwrap());
                    line = lines.next();
                }
                frontmatter = json5::from_str(&json_str).ok();
                lines.map(|l| format!("\n{l}")).collect()
            } else {
                let mut content = String::with_capacity(md.len());
                content.push_str(&line);
                content.push_str(&lines.map(|l| format!("\n{l}")).collect::<String>());
                content
            }
        } else {
            String::new()
        };
        let file_name = &file.file_name().unwrap().to_string_lossy();
        let slug = slugify(file_name.trim_end_matches(".md").trim_start_matches('_'));

        Self {
            slug,
            content,
            frontmatter,
            hidden: file_name.starts_with("_"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontMatter {
    title: Option<String>,
    tags: Option<Vec<String>>,
    date: Option<String>,
    description: Option<String>,
}

#[test]
fn from_md_test() {
    use std::str::FromStr;
    let string = r#"
    ---
    {
        title: 'Hello World',
        tags: ['a', 'b'],
        description: 'Good day, wonderful planet'
    }
    ---

    # My content
    "#;

    let path = PathBuf::from_str("my-file.md").unwrap();

    let p = Post::from_md(&path, string);

    assert_eq!(p.content.trim(), "# My content");
    assert_eq!(p.slug, "my-file");
    assert_eq!(p.hidden, false);
    assert_eq!(p.frontmatter.clone().unwrap().title.unwrap(), "Hello World");
    assert_eq!(p.frontmatter.clone().unwrap().tags.unwrap(), &["a", "b"]);
    assert!(p.frontmatter.clone().unwrap().date.is_none());
    assert_eq!(
        p.frontmatter.unwrap().description.unwrap(),
        "Good day, wonderful planet"
    );
}

#[test]
fn from_md_test2() {
    use std::str::FromStr;
    let string = r#"
    ---
    {
        title: 'Hello World (Hidden)',
        tags: ['a', 'c'],
        description: 'Good day, wonderful planet'
    }
    ---
    # This is also content
    "#;

    let path = PathBuf::from_str("_my-hidden-file.md").unwrap();

    let p = Post::from_md(&path, string);

    assert_eq!(p.content.trim(), "# This is also content");
    assert_eq!(p.slug, "my-hidden-file");
    assert_eq!(p.hidden, true);
    assert_eq!(
        p.frontmatter.clone().unwrap().title.unwrap(),
        "Hello World (Hidden)"
    );
    assert_eq!(p.frontmatter.clone().unwrap().tags.unwrap(), &["a", "c"]);
    assert!(p.frontmatter.clone().unwrap().date.is_none());
    assert_eq!(
        p.frontmatter.unwrap().description.unwrap(),
        "Good day, wonderful planet"
    );
}

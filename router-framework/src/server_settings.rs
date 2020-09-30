use std::collections::HashMap;
use std::convert::From;
use std::ops::Deref;
use std::path::PathBuf;

// we create a content struct that holds our content
// could be a file or a string (raw_content)
#[derive(Debug, Clone)]
pub struct Content {
    pub raw_content: Option<String>,
    pub file: Option<String>,
}

impl Content {
    // check if the entry is a file
    pub fn is_file(&self) -> bool {
        self.file.is_some()
    }
}
// we impl From<&str> for content so we can create a Content struct
// from a &str (for raw_content)
impl From<&str> for Content {
    fn from(content: &str) -> Self {
        Content {
            raw_content: Some(content.to_owned()),
            file: None,
        }
    }
}

// we impl From<PathBuf> for content so we can create a Content struct
// from a PathBuf (for files)
impl From<PathBuf> for Content {
    fn from(content: PathBuf) -> Self {
        Content {
            raw_content: None,
            file: content.to_str().map(|e| e.to_owned()),
        }
    }
}

impl Default for Content {
    fn default() -> Self {
        Self::new()
    }
}

impl Content {
    // we make a function to return an empty instance of Content
    pub fn new() -> Self {
        Content {
            raw_content: None,
            file: None,
        }
    }
}

// our settings struct holds the content along with the url at which
// they'll be available
pub struct Settings {
    pub pages: HashMap<String, Content>,
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Settings {
    // we make a function to return an empty instance of Settings
    pub fn new() -> Self {
        Settings {
            pages: HashMap::new(),
        }
    }
    // we create a method that takes in two strs, content and location
    // and we store them in the hashmap
    pub fn content(&mut self, at: &str, content: &str) -> &mut Self {
        // here we make a content struct from a &str
        let content = Content::from(content);
        // insert it into the hashmap
        self.pages.insert(at.to_owned(), content);
        self
    }
    // we create another function but this time, instead of accepting a &str
    // for content, we accept a std::path::PathBuf which tells that this is a file
    // located a
    pub fn file(&mut self, at: &str, content: PathBuf) -> &mut Self {
        // here we make a content struct from a PathBuf
        let content = Content::from(content);
        // insert it into the hashmap
        self.pages.insert(at.to_owned(), content);
        self
    }
}

impl Deref for Settings {
    type Target = HashMap<String, Content>;
    fn deref(&self) -> &Self::Target {
        &self.pages
    }
}

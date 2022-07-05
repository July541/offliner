use std::{time::SystemTime, collections::HashSet, path::PathBuf};

use url::Url;
use uuid::{Uuid, UuidVersion};

#[derive(Debug)]
pub enum FileType {
    HTML,
    PDF
}

#[derive(Debug)]
struct Tag {
    tag_id: Uuid,
    tag_name: String,
    modify_time: SystemTime,
}

#[derive(Debug)]
pub struct FileAttr {
    file_type: FileType,
    title: Option<String>,
    tags: HashSet<Tag>,
    author: Option<String>,
    // source_url: Url, // Should only exist in db
    /// Renered short url
    rendered: Option<String>
}

impl FileAttr {
    pub fn new(file_type: FileType) -> Self {
        Self {
            file_type,
            title: None,
            tags: HashSet::new(),
            author: None,
            rendered: None
        }
    }

    pub fn new_html() -> Self {
        Self::new(FileType::HTML)
    }

    pub fn new_pdf() -> Self {
        Self::new(FileType::PDF)
    }
}

#[derive(Debug)]
pub struct File {
    modify_time: SystemTime,
    uuid: uuid::Uuid,
    attrs: FileAttr,
    relative_path: PathBuf,
}

impl File {
    pub fn new(attrs: FileAttr, relative_path: PathBuf) -> Self {
        Self {
            attrs,
            modify_time: SystemTime::now(),
            uuid: Uuid::new_v4(),
            relative_path
        }
    }
}
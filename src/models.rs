use std::{cell::RefCell, collections::HashMap, fmt::Display};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    Exit,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed

}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Open => write!(f, "OPEN"),
            Status::InProgress => write!(f, "IN PROGRESS"),
            Status::Resolved => write!(f, "RESOLVED"),
            Status::Closed => write!(f, "Closed"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Epic { 
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>

}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self { 
            name,
            description,
            status: Status::Open,
            stories: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Story { 
    pub name: String,
    pub description: String,
    pub status: Status
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self { 
            name,
            description,
            status: Status::Open,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DBState { 
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>

    // pub last_item_id: RefCell<u32>,
    // pub epics: RefCell<HashMap<u32, Epic>>,
    // pub stories: RefCell<HashMap<u32, Story>>

}

impl DBState {
    pub fn new() -> Self {

        Self { 
            last_item_id: 0,
            epics:HashMap::new(),
            stories:HashMap::new(),
        }

        // Self { 
        //     last_item_id: RefCell::new(0),
        //     epics:RefCell::new(HashMap::new()),
        //     stories:RefCell::new(HashMap::new()),
        // }
    }
}
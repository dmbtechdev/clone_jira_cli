use std::{fs, fs::File, ops::Index};
use anyhow::{anyhow, Context, Ok, Result};

use crate::models::{DBState, Epic, Story, Status};
// use crate::models::*;

pub trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}
#[derive(Debug)]
pub struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {

        let db_content = fs::read_to_string(&self.file_path)?;
        serde_json::from_str(&db_content).with_context(|| format!("Error while reading or parsing"))
        // Ok(db)

        // let file = File::open(&self.file_path).with_context(|| format!("The path is: \"{}\"", &self.file_path))?;
        // serde_json::from_reader(file).with_context(|| format!("Error while reading or parsing"))
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {

        // fs::write(&self.file_path, &serde_json::to_vec(db_state)?).with_context(|| format!("Error while writing"))
        // Ok(())

        let file = File::create(&self.file_path).with_context(|| format!("The path is: \"{}\"", &self.file_path))?;
        serde_json::to_writer_pretty(file, db_state).with_context(|| format!("Error while writing"))
    }
}


// for CRUD methods
pub struct JiraDatabase {
    pub database: Box<dyn Database>
}

// CRUD methods - (AGER) - create/add, read/get, update/edit, delete/remove 
impl JiraDatabase {
    
    pub fn new(file_path:String) -> Self {
        Self { 
            database: Box::new(JSONFileDatabase {file_path})
        }
    }

    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_db()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {

        let mut db = self.database.read_db().with_context(|| format!("Error while reading the database"))?;
        db.last_item_id += 1;
        db.epics.insert(db.last_item_id, epic);
        self.database.write_db(&db).with_context(|| format!("Error while writing"))?;
        Ok(db.last_item_id)

        // with DBState fields wrapped with RefCell

        // let db_state = self.read_db()?;
        // *db_state.last_item_id.borrow_mut() += 1;
        // db_state.epics.borrow_mut().insert(*db_state.last_item_id.borrow(), epic);
        // self.database.write_db(&db_state)?;
        // println!("Epic no:{} aded",*db_state.last_item_id.borrow());
        // let a = *db_state.last_item_id.borrow();
        // Ok(a)
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {

        let mut db = self.database.read_db().with_context(|| format!("Error while reading the database"))?;
        db.last_item_id +=1;
        db.stories.insert(db.last_item_id, story);
        db.epics.get_mut(&epic_id).ok_or_else(|| anyhow!("could not find epic no:{epic_id} in database!. Story is added but no Epic is updated!!"))?.stories.push(db.last_item_id);
        self.database.write_db(&db).with_context(|| format!("Error while writing"))?;
        Ok(db.last_item_id)

        // with DBState fields wrapped with RefCell
        
        // let db_state = self.read_db()?;
        // *db_state.last_item_id.borrow_mut() += 1;
        // db_state.stories.borrow_mut().insert(*db_state.last_item_id.borrow(), story);
        // self.database.write_db(&db_state)?;
        // println!("Story no:{} aded", *db_state.last_item_id.borrow());
        // let a = *db_state.last_item_id.borrow();
        // Ok(a)
    }

    pub fn delete_epic(&self, epic_id:u32) -> Result<()> {

        let mut db = self.database.read_db().with_context(|| format!("Error while reading the database"))?;

        //delete stories that are included in the epic, too
        
        if db.epics.contains_key(&epic_id) {
            
            for story_id in &db.epics.get(&epic_id).ok_or_else(|| anyhow!("could not find epic in database!"))?.stories {
                db.stories.remove(story_id);
            }
            
            db.epics.remove(&epic_id);
            self.database.write_db(&db).with_context(|| format!("Error while writing"))?;
        } else {
            return Err(anyhow!("Epic no: {} is not present", epic_id))
        }

        Ok(())
        // with DBState fields wrapped with RefCell
        
        // let db_state = self.read_db()?;
        // if db_state.epics.borrow().contains_key(&epic_id) {
        //     db_state.epics.borrow_mut().remove(&epic_id);
        //     self.database.write_db(&db_state)?;
        //     println!("Epic no:{} removed", epic_id);
        //     Ok(())
        // } else {
        //     Err(anyhow!("Epic no: {} is not present", epic_id))
        // }
    }

    pub fn delete_story(&self, epic_id: u32, story_id:u32) -> Result<()> {
        
        let mut db = self.database.read_db().with_context(|| format!("Error while reading the database"))?;

        // remove story from epics stories
        let epic = db.epics
                            .get_mut(&epic_id)
                            .ok_or_else(|| anyhow!("could not find epic in database!"))?;
                            
        let story_index = epic.stories.iter()
                                    .position(|id| id == &story_id)
                                    .ok_or_else(|| anyhow!("story id not found in epic stories vector"))?;

        epic.stories.remove(story_index);


        // if same story exists in more than one epic, which is not possible in this scenerio
        // for (_, epic) in db.epics.iter_mut() {
            
        //     let story_index = epic.stories
        //         .iter()
        //         .position(|id| id == &story_id)
        //         .ok_or_else(|| anyhow!("story id not found in epic stories vector"))?;

        //     epic.stories.remove(story_index);

        //     }


        // for (_, epic) in db.epics.iter_mut() {
        //     let mut id_delete:usize = 0;
        //     for (index,id) in epic.stories.iter().enumerate() {
        //         if *id == story_id {
        //             id_delete = index;
        //         }
        //     }
        //     epic.stories.remove(id_delete);
        // }
        
        // remove story
        if db.stories.contains_key(&story_id) {
            db.stories.remove(&story_id);
            self.database.write_db(&db).with_context(|| format!("Error while writing"))?;
            Ok(())
        } else {
            Err(anyhow!("Story no: {} is not present", story_id))
        }

        // with DBState fields wrapped with RefCell       

        // let db_state = self.read_db()?;
        // if db_state.stories.borrow().contains_key(&story_id) {
        //     db_state.stories.borrow_mut().remove(&story_id);
        //     self.database.write_db(&db_state)?;
        //     println!("Story no:{} removed", story_id);
        //     Ok(())
        // } else {
        //     Err(anyhow!("Story no: {} is not present", story_id))
        // }
    }

    pub fn update_epic_status(&self, epic_id:u32, status: Status) -> Result<()> {

        let mut db = self.database.read_db().with_context(|| format!("Error while reading the database"))?;

        db.epics.get_mut(&epic_id).ok_or_else(|| anyhow!("Epic id not: {epic_id} found"))?.status = status;

        self.database.write_db(&db)?;

        Ok(())

        // if db.epics.contains_key(&epic_id) {
        //     db.epics.get_mut(&epic_id);
        //     self.database.write_db(&db).with_context(|| format!("Error while writing"))?;
        //     Ok(())
        // } else {
        //     Err(anyhow!("Epic no: {} is not present", epic_id))
        // }


        // with DBState fields wrapped with RefCell       

        // let db_state = self.read_db()?;
        // if db_state.epics.borrow().contains_key(&epic_id) {
        //     if let Some(epic) = db_state.epics.borrow_mut().get_mut(&epic_id) {
        //         epic.status = status;
        //     }
        //     self.database.write_db(&db_state)?;
        //     println!("Epic no:{} updated", epic_id);
        //     Ok(())
        // } else {
        //     Err(anyhow!("Epic no: {} is not present", epic_id))
        // }        
    }

    pub fn update_story_status(&self, story_id:u32, status: Status) -> Result<()> {

        let mut db = self.database.read_db().with_context(|| format!("Error while reading the database"))?;

        db.stories.get_mut(&story_id).ok_or_else(|| anyhow!("Story id not: {story_id} found"))?.status = status;

        self.database.write_db(&db)?;

        Ok(())

        // with DBState fields wrapped with RefCell       

        // let db_state = self.read_db()?;
        // if db_state.stories.borrow().contains_key(&story_id) {
        //     if let Some(story) = db_state.stories.borrow_mut().get_mut(&story_id) {
        //         story.status = status;
        //     }
        //     self.database.write_db(&db_state)?;
        //     println!("Story no:{} updated", story_id);
        //     Ok(())
        // } else {
        //     Err(anyhow!("Story no: {} is not present", story_id))
        // }
    }
}

pub mod test_utils {
    use std::{cell::RefCell, collections::HashMap};

    use super::*;

    pub struct MockDB {
        last_written_state: RefCell<DBState>
    }

    impl MockDB {
        pub fn new() -> Self {
            Self { last_written_state: RefCell::new(DBState { last_item_id: 0, epics: HashMap::new(), stories: HashMap::new() }) }
        }    
    }

    impl Database for MockDB {
        fn read_db(&self) -> Result<DBState> {
            // TODO: fix this error by deriving the appropriate traits for Story
            let state = self.last_written_state.borrow().clone();
            Ok(state)
        }

        fn write_db(&self, db_state: &DBState) -> Result<()> {
            let latest_state = &self.last_written_state;
            // TODO: fix this error by deriving the appropriate traits for DBState
            *latest_state.borrow_mut() = db_state.clone();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::test_utils::MockDB;

    #[test]
    fn create_epic_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());

        // TODO: fix this error by deriving the appropriate traits for Epic
        let result = db.create_epic(epic.clone());
        
        assert_eq!(result.is_ok(), true);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 1;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&id), Some(&epic));
    }

    #[test]
    fn create_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let story = Story::new("".to_owned(), "".to_owned());

        let non_existent_epic_id = 999;

        let result = db.create_story(story, non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn create_story_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        // TODO: fix this error by deriving the appropriate traits for Story
        let result = db.create_story(story.clone(), epic_id);
        assert_eq!(result.is_ok(), true);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 2;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&epic_id).unwrap().stories.contains(&id), true);
        assert_eq!(db_state.stories.get(&id), Some(&story));
    }

    #[test]
    fn delete_epic_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };

        let non_existent_epic_id = 999;

        let result = db.delete_epic(non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_epic_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_epic(epic_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id), None);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn delete_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);
        
        let story_id = result.unwrap();

        let non_existent_epic_id = 999;
        
        let result = db.delete_story(non_existent_epic_id, story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_error_if_story_not_found_in_epic() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let non_existent_story_id = 999;
        
        let result = db.delete_story(epic_id, non_existent_story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_story(epic_id, story_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id).unwrap().stories.contains(&story_id), false);
        // assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn update_epic_status_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };

        let non_existent_epic_id = 999;

        let result = db.update_epic_status(non_existent_epic_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_epic_status_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.update_epic_status(epic_id, Status::Closed);

        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);
    }

    #[test]
    fn update_story_status_should_error_if_invalid_story_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };

        let non_existent_story_id = 999;

        let result = db.update_story_status(non_existent_story_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_story_status_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);

        let story_id = result.unwrap();

        let result = db.update_story_status(story_id, Status::Closed);

        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        assert_eq!(db_state.stories.get(&story_id).unwrap().status, Status::Closed);
    }

    mod database {
        use std::collections::HashMap;
        use std::io::Write;
        
        use super::*;
        
        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase { file_path: "INVALID_PATH".to_owned() };
            assert_eq!(db.read_db().is_err(), true);
        }
        
        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        
            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();
        
            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };
        
            let result = db.read_db();
        
            assert_eq!(result.is_err(), true);
        }
        
        #[test]
        fn read_db_should_parse_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        
            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();
        
            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };
        
            let result = db.read_db();
        
            assert_eq!(result.is_ok(), true);
        }
        
        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        
            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();
        
            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };
        
            let story = Story { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open };
            let epic = Epic { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open, stories: vec![2] };
        
            let mut stories = HashMap::new();
            stories.insert(2, story);
        
            let mut epics = HashMap::new();
            epics.insert(1, epic);
        
            let state = DBState { last_item_id: 2, epics, stories };
        
            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();
        
            assert_eq!(write_result.is_ok(), true);
            assert_eq!(read_result, state);
        }
    }
}
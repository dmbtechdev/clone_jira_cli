mod models;
mod db;
mod ui;
mod io_utils;
mod navigator;

use models::*;
use db::*;
use colored::*;
use navigator::*;
use io_utils::*;


use std::{cell::RefCell, collections::HashMap, rc::Rc};


fn main() {

    // TODO: create database and navigator

    let file_path = "./data/db_test.json".to_owned();
    let db = Rc::new(JiraDatabase::new(file_path));
    let mut navigator = Navigator::new(Rc::clone(&db));

    // let mut navigator = Navigator::new(Rc::new(JiraDatabase::new(file_path)));
    
    // let db = JSONFileDatabase { file_path }.read_db();
    loop {
        clearscreen::clear().unwrap();

        // if let Some(page) = navigator.get_current_page() {
        //     if let Err(error) = page.draw_page() {
        //         println!(
        //             "Error rendering page: {}\nPress any key to continue...",
        //             error
        //         );
        //         wait_for_key_press();
        //     };

        //     let user_input = get_user_input();

        //     match page.handle_input(user_input.trim()) {
        //         Err(error) => {
        //             println!(
        //                 "Error getting user input: {}\nPress any key to continue...",
        //                 error
        //             );
        //             wait_for_key_press();
        //         }
        //         Ok(action) => {
        //             if let Some(action) = action {
        //                 if let Err(error) = navigator.handle_action(action) {
        //                     println!("Error handling processing user input: {}\nPress any key to continue...", error);
        //                     wait_for_key_press();
        //                 }
        //             }
        //         }
        //     }
        // } else {
        //     break;
        // }



        if let Some(current_page) = navigator.get_current_page() {
            if let Err(error) = current_page.draw_page() {
                println!("{}{}{}",
                ("Error rendering page: \n").red(),
                error,
                ("\n Press any key to continue...").red(),
                );
                wait_for_key_press();
            }
            let user_input = get_user_input();
            // println!("{:?}", user_input);
            // if let Ok(Some(action)) = current_page.handle_input(user_input.trim()) {
            //     // println!("{:?}", action);
            //     // wait_for_key_press();
            //     navigator.handle_action(action).unwrap();
            // }
            match current_page.handle_input(user_input.trim()) {
                Err(error)=> {
                    println!("{}{}{}",
                    ("Error rendering page: \n").red(),
                    error,
                    ("\n Press any key to continue...").red(),
                    );
                    wait_for_key_press();    
                }
                Ok(act) => {
                    if let Some(action) = act {
                        if let Err(error) = navigator.handle_action(action) {
                            println!("{}{}{}",
                                ("Error rendering page: \n").red(),
                                error,
                                ("\n Press any key to continue...").red(),
                            );
                            wait_for_key_press();        
                        }
                        
                    }
                },
            }

        } else {
            break
        }

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        // 2. render page
        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action
    }


    
    
    
    // match db {
    //     Ok(db) => println!("\t{}","DB is available".green() ),
    //     Err(e) => {
    //         println!("{} {}","Error reading db:".red(),e.to_string().red());
    //         let db_init = DBState::new();
    //         let file_path = "./data/db_test.json".to_owned();
    //         let saved = JSONFileDatabase { file_path }.write_db(&db_init);
    //         match saved {
    //             Ok(_) => println!("{}","DB initialized!".green()),
    //             Err(e) => {
    //                 println!("{} {}","Error writing to db:".red(),e.to_string().red());
    //                 return
    //             },
    //         }
    //     },
    // }

    // let db_test = DBState::new(    
    //     RefCell::new(HashMap::from([(2, epic1)])),
    //     RefCell::new(HashMap::from([(3, story1)]))
    // );    

    // let file_path = "./data/db_test.json".to_owned();
    // let jiraDb = JiraDatabase::new(file_path);   
    
    // let epic1 = Epic::new(
    //     "Epic 1".to_string(),
    //     "Epic 1 description".to_string()
    // );
    
    // let story1 = Story::new(        
    //     "Story 1".to_string(),
    //     "Story 1 description".to_string()
    // );

    // jiraDb.create_epic(epic1);
    // jiraDb.create_story(story1);
    // match jiraDb.delete_epic(5) {
    //     Ok(_) => (),
    //     Err(e) => println!("{} {}","Error removing epic:".red(), e.to_string().red()),
    // };
    // match jiraDb.delete_story(6) {
    //     Ok(_) => (),
    //     Err(e) => println!("{} {}","Error removing story:".red(), e.to_string().red()),
    // }

}

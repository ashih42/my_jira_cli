use std::rc::Rc;

mod db;
mod io_utils;
mod models;
mod navigator;
mod ui;

use db::*;
use io_utils::*;
use navigator::*;

fn main() {
    let database = JiraDatabase::new("data/db.json".to_owned());
    let mut navigator = Navigator::new(Rc::new(database));

    loop {
        clearscreen::clear().unwrap();

        // 1. Get the current page from navigator.
        match navigator.get_current_page() {
            // If there is no current page, exit the loop.
            None => break,

            Some(page) => {
                // 2. Render the page.
                if let Err(err) = page.draw_page() {
                    println!("Error drawing page: {}\nPress any key to continue...", err);
                    wait_for_key_press();
                    break;
                }

                // 3. Get user input.
                let ch = get_user_input_char();

                // 4. Pass input to page's input handler.
                match page.handle_input_char(ch) {
                    Err(err) => {
                        println!(
                            "Error handling input: {}\nPress any key to continue...",
                            err
                        );
                        wait_for_key_press();
                    }
                    Ok(action) => {
                        // 5. Let the navigator process the action.
                        if let Err(err) = navigator.handle_action(action) {
                            println!(
                                "Error handling action: {}\nPress any key to continue...",
                                err
                            );
                            wait_for_key_press();
                        }
                    }
                }
            }
        }
    }
}

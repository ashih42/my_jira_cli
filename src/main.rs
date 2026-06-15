use std::rc::Rc;

mod db;
mod io_utils;
mod models;
mod navigator;
mod ui;

use anyhow::Error;
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
                    break show_error(err);
                }

                // 3. Get user input.
                let ch = get_user_input_char();

                // 4. Pass input to page's input handler.
                match page.handle_input_char(ch) {
                    Err(err) => {
                        show_error(err);
                    }
                    Ok(action) => {
                        // 5. Let the navigator process the action.
                        if let Err(err) = navigator.handle_action(action) {
                            show_error(err);
                        }
                    }
                }
            }
        }
    }
}

/// Note: If any error happened, it's probably coming from database file read/write operations.
fn show_error(err: Error) {
    println!(
        "Error handling action: {}\nPress any key to continue...",
        err
    );
    wait_for_key_press();
}

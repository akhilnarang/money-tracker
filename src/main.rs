#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;

use std::ops::DerefMut;

use rocket::serde::json::Json;
use uuid::Uuid;

use db::{get_expenses, insert_expense};
use models::{Expense, ExpenseInsert};

use crate::db::get_now;

mod db;
mod models;
mod schema;
mod tests;

#[get("/")]
fn index() -> &'static str {
    "Expense tracker 👀"
}

#[get("/")]
fn list_expenses() -> Json<Vec<Expense>> {
    Json(get_expenses())
}

#[post("/", data = "<expense>")]
fn add_expense(mut expense: Json<ExpenseInsert>) -> Json<Expense> {
    if expense.id.is_none() {
        expense.id = Option::from(Uuid::new_v4());
    }
    let now = get_now();
    expense.created = Option::from(now);
    expense.last_updated = Option::from(now);

    Json(insert_expense(expense.deref_mut()))
}

#[launch]
fn rocket() -> _ {
    println!("Starting up!");
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1/", routes![list_expenses, add_expense])
}

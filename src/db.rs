use std::env;
use std::time::SystemTime;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::select;
use dotenv::dotenv;

use crate::models::{Expense, ExpenseInsert};
use crate::schema::expenses;

pub fn establish_db_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Failed to connect to {}", &database_url))
}

pub fn get_now() -> SystemTime {
    let connection = establish_db_connection();
    match select(diesel::dsl::now).get_result::<SystemTime>(&connection) {
        Ok(n) => n,
        _ => panic!(),
    }
}

pub fn get_expenses() -> Vec<Expense> {
    let connection = establish_db_connection();
    expenses::table
        .load::<Expense>(&connection)
        .expect("Error fetching expenses")
}

pub fn insert_expense(expense: &ExpenseInsert) -> Expense {
    let connection = establish_db_connection();
    diesel::insert_into(expenses::table)
        .values(expense)
        .get_result(&connection)
        .expect("Error inserting record")
}

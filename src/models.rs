use std::fmt;
use std::time::SystemTime;

use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::expenses;

#[derive(Deserialize, Serialize, Queryable)]
pub struct Expense {
    id: Uuid,
    pub description: String,
    pub amount: f32,
    pub payment_method: String,
    created: SystemTime,
    last_updated: SystemTime,
}

#[derive(Deserialize, Insertable, Serialize)]
#[table_name = "expenses"]
pub struct ExpenseInsert {
    pub id: Option<Uuid>,
    pub description: String,
    pub amount: f32,
    pub payment_method: String,
    pub created: Option<SystemTime>,
    pub last_updated: Option<SystemTime>,
}

impl fmt::Display for Expense {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Description: {}\nAmount: {}\nPayment Method: {}",
            self.description, self.amount, self.payment_method
        )
    }
}

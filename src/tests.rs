#[cfg(test)]
mod test {
    use crate::models::{Expense, ExpenseInsert};
    use crate::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    use rocket::serde::json::serde_json;

    #[test]
    fn index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Expense tracker 👀");
    }

    #[test]
    fn check_empty() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/v1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response_body = match response.into_json::<Vec<Expense>>() {
            Some(r) => r,
            _ => panic!(),
        };
        assert_eq!(response_body.len(), 0)
    }

    #[test]
    fn insert() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let description = "Test description";
        let amount = 3.14;
        let payment_method = "UPI";
        let request_body = ExpenseInsert {
            id: None,
            description: description.parse().unwrap(),
            amount,
            payment_method: payment_method.parse().unwrap(),
            created: None,
            last_updated: None,
        };
        let body = match serde_json::to_string(&request_body) {
            Ok(body) => body,
            Err(error) => panic!("Error converting body to json!\n{}", error),
        };
        let response = client
            .post("/api/v1")
            .header(ContentType::JSON)
            .body(body)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let response_body = match response.into_json::<Expense>() {
            Some(r) => r,
            _ => panic!(),
        };
        assert_eq!(response_body.description, description);
        assert_eq!(response_body.amount, amount);
        assert_eq!(response_body.payment_method, payment_method);
    }
}

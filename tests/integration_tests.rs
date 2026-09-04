#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use diesel::prelude::*;

    #[actix_web::test]
    async fn test_create_account() {
        let app = test::init_service(App::new().configure(application::config)).await;
        let new_account = web::Json(NewAccount {
            account_number: "1234567890".to_string(),
            holder_name: "John Doe".to_string(),
            balance: 100.0,
            account_type: "Savings".to_string(),
        });

        let req = test::TestRequest::post()
           .uri="/accounts"
           .set_json(&new_account)
           .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
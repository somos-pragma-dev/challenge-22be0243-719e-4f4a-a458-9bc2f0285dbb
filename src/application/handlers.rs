use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::{Account, NewAccount};
use crate::infrastructure::establish_connection;

pub async fn create_account(new_account: web::Json<NewAccount>) -> impl Responder {
    use crate::infrastructure::schema::accounts::dsl::*;

    let conn = establish_connection();

    let new_account = NewAccount {
        account_number: new_account.account_number.clone(),
        holder_name: new_account.holder_name.clone(),
        balance: new_account.balance,
        account_type: new_account.account_type.clone(),
    };

    diesel::insert_into(accounts)
       .values(&new_account)
       .execute(&conn)
       .expect("Error saving new account");

    HttpResponse::Ok().json(new_account)
}

pub async fn get_account(info: web::Path<(i32,)>) -> impl Responder {
    use crate::infrastructure::schema::accounts::dsl::*;

    let conn = establish_connection();
    let id = info.0;

    accounts
       .filter(id.eq(id))
       .first::<Account>(&conn)
       .optional()
       .map(|account| {
            match account {
                Some(acc) => HttpResponse::Ok().json(acc),
                None => HttpResponse::NotFound().finish(),
            }
        })
       .expect("Error loading account")
}

pub async fn update_account(info: web::Path<(i32,)>, new_account: web::Json<NewAccount>) -> impl Responder {
    use crate::infrastructure::schema::accounts::dsl::*;

    let conn = establish_connection();
    let id = info.0;

    diesel::update(accounts.find(id))
       .set(&new_account)
       .execute(&conn)
       .expect("Error updating account");

    HttpResponse::Ok().json(new_account)
}

pub async fn delete_account(info: web::Path<(i32,)>) -> impl Responder {
    use crate::infrastructure::schema::accounts::dsl::*;

    let conn = establish_connection();
    let id = info.0;

    diesel::delete(accounts.find(id))
       .execute(&conn)
       .expect("Error deleting account");

    HttpResponse::Ok().finish()
}
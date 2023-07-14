use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::domain::{CompanyName, Email, PeopleName};

#[derive(Deserialize)]
struct ParamsAccount {
    company_name: CompanyName,
    admin_lastname: PeopleName,
    admin_firstname: PeopleName,
    admin_email: Email,
}

/*
impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}
*/

#[post("/accounts")]
async fn create_account(params_account: web::Json<ParamsAccount>) -> impl Responder {
    println!(
        "{:?}-{:?}-{:?}-{:?}",
        params_account.company_name.as_ref(),
        params_account.admin_firstname.as_ref(),
        params_account.admin_lastname.as_ref(),
        params_account.admin_email.as_ref()
    );

    HttpResponse::Ok().finish()
}

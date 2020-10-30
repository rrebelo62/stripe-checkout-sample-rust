use actix_web::{HttpResponse, web, Result};
use actix_web::error::{ ErrorInternalServerError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest;
use dotenv;
use stripe::CheckoutSession;
use crate::model::CreateCheckoutSessionResponse;

use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref STRIPE_SECRET_KEY: String = {
		let string_secret_key = dotenv::var("STRIPE_SECRET_KEY").expect("Can't get stripe secret key from environment.");
		string_secret_key
	};
	static ref SESSIONS_MAP: RwLock< HashMap<String, CheckoutSession> > = RwLock::new( HashMap::new());
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Setup{
	pro_price:String,
	basic_price:String,
	publishable_key:String,
}

pub async fn setup() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Setup {
		pro_price: dotenv::var("PRO_PRICE_ID").unwrap(),
		basic_price:dotenv::var("BASIC_PRICE_ID").unwrap(),
		publishable_key:dotenv::var("STRIPE_PUBLISHABLE_KEY").unwrap(),
    }))
}

//[HttpPost("customer-portal")]
//#[post("webhook")]
//async fn webhook()->Result<HttpResponse>{
//	Ok(HttpResponse::Ok().body(dotenv::var("STRIPE_WEBHOOK_SECRET")))	
//}
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct CreateCheckoutSessionParm {
    priceId: String,
}

async fn post_form( url : String, parms:HashMap<&str, &str> )->Result<String, reqwest::Error>
{
	let stripe_secret_key = STRIPE_SECRET_KEY.clone();
	// using reqwest here because actix_web::Client doesn't implement https and authentication
	let client = reqwest::Client::new();
	let result = client.post(&url)
		.basic_auth(stripe_secret_key, Option::<String>::None)
		.form(&parms)
		.send().await;
	match result {
		Ok(resp)=> Ok(resp.text().await?),
		Err(err)=> Err(err)
	}
}

async fn create_checkout_session_internal( id : String )->Result<HttpResponse>{
	// from https://stripe.com/docs/api/checkout/sessions/create
	let mut session_parms :HashMap<&str, &str> = HashMap::new();
	session_parms.insert("success_url", "http://localhost:4242/success.html");
	session_parms.insert("cancel_url", "http://localhost:4242/canceled.html");
	session_parms.insert("payment_method_types[0]", "card");
	session_parms.insert("mode", "subscription");
	session_parms.insert("line_items[0][price]", &id);
	session_parms.insert("line_items[0][quantity]", "1");
	let result = post_form("https://api.stripe.com/v1/checkout/sessions".to_string(), session_parms).await;
	match result{
		Ok(json) => {
			println!("{:?}", json);
			let session : CheckoutSession = serde_json::from_str(&json).unwrap();
			let sessions_map = SESSIONS_MAP.write().unwrap();
			return Ok( HttpResponse::Ok().json( CreateCheckoutSessionResponse{ session_id: session.id.to_string(), }));
		},
		Err(err) => Err(ErrorInternalServerError(err))
	}
}

pub async fn create_checkout_session(parm: web::Json<CreateCheckoutSessionParm> ) ->Result<HttpResponse>{
	//println!("{0}", parm.priceId);
	return create_checkout_session_internal(parm.priceId.clone()).await;
}


#[cfg(test)]
mod tests{
	use super::*;
	use std::sync::Once;
	macro_rules! aw {
		($e:expr) => {
			tokio_test::block_on($e)
		};
	  }


static START: Once = Once::new();

	#[test]
	fn create_session_success(){
		let result = aw!(create_checkout_session_internal("price_1HfAogI5kR79VI05bf1wpJVx".to_string()));
		if let Ok( r) = result {
			println!("Session id:");
		} else{
			assert!(false);
		}
	}
}
use actix_web::{HttpResponse, HttpRequest, web, Result};
use actix_web::error::{ ErrorInternalServerError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest;
use dotenv;
use crate::model;
use lazy_static::lazy_static;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    static ref STRIPE_SECRET_KEY: String = {
		let string_secret_key = dotenv::var("STRIPE_SECRET_KEY").expect("Can't get stripe secret key from environment.");
		string_secret_key
	};
	static ref SESSIONS_MAP: RwLock< HashMap<String, model::CheckoutSession> > = RwLock::new( HashMap::new());
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

pub async fn checkout_session( req:HttpRequest ) -> Result<HttpResponse>{
	// horrible hack: parsing the uri query because couldn't get actix deserializing it, sorry
	if let Some(query ) = req.uri().query() {
		let session_id_vec:Vec<&str> = query.split("sessionId=").collect();
		if session_id_vec.len() == 2 {
			let session_id= session_id_vec[1];
			let sessions_map = SESSIONS_MAP.read().unwrap();
			if sessions_map.contains_key( session_id ) {
				return Ok( HttpResponse::Ok().json(sessions_map.get( session_id )));
			}
		}
	}
	return Err( ErrorInternalServerError("Can't find such session"));
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct CustomerPortalParm {
    pub customerId: String,
}
#[derive(Serialize)]
pub struct CustomerPortalResponse {
    url: String,
}
pub async fn customer_portal(parm: web::Json<CustomerPortalParm>) -> Result<HttpResponse>{
	//https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#redirect

	let mut session_parms :HashMap<&str, &str> = HashMap::new();
	session_parms.insert("customer", &parm.customerId);
	session_parms.insert("return_url", "http://localhost:4242");
	let result = post_form("https://api.stripe.com/v1/billing_portal/sessions".to_string(), session_parms).await;
	match result{
		Ok(json) => {
			let session : model::CustomerPortalSession = serde_json::from_str(&json).unwrap();
			return Ok( HttpResponse::Ok().json( CustomerPortalResponse{ url: session.url }));
		},
		Err(err) => Err(ErrorInternalServerError(err))
	}
}

pub async fn webhook(parm: web::Json<model::StripeEvent>)->Result<HttpResponse>{
	println!("Webhook notification with type: {0} found for {1}", parm.Type, parm.id);
	if parm.Type == "checkout.session.completed" {
		println!("Session ID {0} completed", parm.id );
	}
	Ok(HttpResponse::Ok().finish())	
}
#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CreateCheckoutSessionParm {
    pub priceId: String,
}

async fn post_form( url : String, parms:HashMap<&str, &str> )->Result<String, reqwest::Error>
{
	let stripe_secret_key = STRIPE_SECRET_KEY.clone();
	// using reqwest here because it seems actix_web::Client doesn't implement authentication
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

// generate a string with timestamp in nanoseconds to use it as a fake customer id,
// to allow access to the customer portal
fn timestamp_as_string() -> String{
	let start = SystemTime::now();
    let since_the_epoch = start
		.duration_since(UNIX_EPOCH)
		.expect("Time went backwards");
	let millis = since_the_epoch.as_nanos();
	format!("{:?}", millis)
}
pub async fn create_checkout_session(parm: web::Json<CreateCheckoutSessionParm> ) ->Result<HttpResponse>{
	// from https://stripe.com/docs/api/checkout/sessions/create
	let mut session_parms :HashMap<&str, &str> = HashMap::new();
	session_parms.insert("success_url", "http://localhost:4242/success.html?session_id={CHECKOUT_SESSION_ID}");
	session_parms.insert("cancel_url", "http://localhost:4242/canceled.html");
	session_parms.insert("payment_method_types[0]", "card");
	session_parms.insert("mode", "subscription");
	session_parms.insert("line_items[0][price]", &parm.priceId );
	session_parms.insert("line_items[0][quantity]", "1");
	//let fake_customer_id = timestamp_as_string();
	//session_parms.insert("customer", &fake_customer_id);
	let result = post_form("https://api.stripe.com/v1/checkout/sessions".to_string(), session_parms).await;
	match result{
		Ok(json) => {
			let session : model::CheckoutSession = serde_json::from_str(&json).unwrap();
			let session_id_string : String = session.id.to_string();
			let mut sessions_map = SESSIONS_MAP.write().unwrap();
			sessions_map.insert(session_id_string.clone(), session);
			return Ok( HttpResponse::Ok().json( model::CreateCheckoutSessionResponse{ session_id: session_id_string, }));
		},
		Err(err) => Err(ErrorInternalServerError(err))
	}
}


#[cfg(test)]
mod tests{
	use actix_web::{test::{self, TestRequest}, App};
	use dotenv::dotenv;
	use super::*;
	use serde_json::json;

	#[actix_rt::test]
		async fn create_session_success(){
			fn init_routes(cfg: &mut web::ServiceConfig) {
				cfg.route("/create-checkout-session", web::post().to(create_checkout_session))
				.route("/checkout-session", web::get().to(checkout_session));
			};

			dotenv().ok();
			let mut app = test::init_service(App::new().configure(init_routes)).await;
			// creates session
			let json_request = json!({"priceId":dotenv::var("BASIC_PRICE_ID").unwrap()});
			let resp = TestRequest::post().uri("/create-checkout-session").set_json(&json_request).send_request(&mut app).await;
			assert!(resp.status().is_success());

			// retrieves created session
			let checkout_session_response : model::CreateCheckoutSessionResponse = test::read_body_json(resp).await;
			let path = format!( "/checkout_session?sessionId={}", checkout_session_response.session_id);
			let test_req = TestRequest::get()
				.uri(&path);
			let resp = test_req.send_request(&mut app).await;
			assert!(resp.status().is_success());
		}
}
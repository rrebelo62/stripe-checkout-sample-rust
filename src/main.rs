extern crate dotenv;
use dotenv::dotenv;
use actix_web::{get, /*post,*/ App, HttpResponse, HttpServer, Result};
use serde::{Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Setup{
	pro_price:String,
	basic_price:String,
	publishable_key:String,
}

#[get("/setup")]
async fn setup() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Setup {
		pro_price: dotenv::var("PRO_PRICE_ID").unwrap(),
		basic_price:dotenv::var("BASIC_PRICE_ID").unwrap(),
		publishable_key:dotenv::var("STRIPE_PUBLISHABLE_KEY").unwrap(),
    }))
}
//[HttpGet("checkout-session")]
//[HttpPost("customer-portal")]
//#[post("webhook")]
//async fn webhook()->Result<HttpResponse>{
//	Ok(HttpResponse::Ok().body(dotenv::var("STRIPE_WEBHOOK_SECRET")))	
//}
//[HttpPost("create-checkout-session")]

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	use actix_files as fs;

	dotenv().ok();
	/*let keys= ["STRIPE_SECRET_KEY", "STRIPE_PUBLISHABLE_KEY", "STRIPE_WEBHOOK_SECRET",
		"DOMAIN", "BASIC_PRICE_ID", "PRO_PRICE_ID", "STATIC_DIR"];
	for key in &keys {
		let value = dotenv::var(key).unwrap();
		println!("{}: {}", key, value);
	}*/

	HttpServer::new(|| {
		let client_folder = dotenv::var("STATIC_DIR").unwrap();
		App::new().service(setup)
			.service(fs::Files::new("/", client_folder).index_file("index.html"))
	})
	.bind("127.0.0.1:4242")?
	.run()
	.await
}


use dotenv::dotenv;
use actix_web::{HttpServer, App, web};
mod model;
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	use actix_files as fs;

	dotenv().ok();
	controllers::ask_password();
	/*let keys= ["STRIPE_SECRET_KEY", "STRIPE_PUBLISHABLE_KEY", "STRIPE_WEBHOOK_SECRET",
		"DOMAIN", "BASIC_PRICE_ID", "PRO_PRICE_ID", "STATIC_DIR"];
	for key in &keys {
		let value = dotenv::var(key).unwrap();
		println!("{}: {}", key, value);
	}*/

	HttpServer::new(|| {
		let client_folder = dotenv::var("STATIC_DIR").unwrap();
		App::new()
		.route("/create-checkout-session", web::post().to(controllers::create_checkout_session))
		.route("/setup", web::get().to(controllers::setup))
			.service(fs::Files::new("/", client_folder).index_file("index.html"))
	})
	.bind("127.0.0.1:4242")?
	.run()
	.await
}


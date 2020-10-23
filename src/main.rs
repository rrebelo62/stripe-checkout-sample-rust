//use actix_web::{HttpRequest, Result};
//use std::path::PathBuf;

/*async fn index(req:HttpRequest) -> Result<NamedFile> {
	let path: PathBuf = req.match_info().query("filename").parse().unwrap();
	//let pathStr:String = path.into_os_string().into_string().unwrap();
	//println!("path: {0}", pathStr);
	Ok(NamedFile::open(path)?)
}*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	use actix_web::{/*web,*/ App, HttpServer};
	use actix_files as fs;

	HttpServer::new(|| {
	    //App::new().route("/{filename:.*}", web::get().to(index))
		App::new().service(fs::Files::new("/", "client").index_file("index.html"))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}


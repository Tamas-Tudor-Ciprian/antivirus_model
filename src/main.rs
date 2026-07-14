mod utils;

use std::fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse:: Ok().body("Yes your server is working")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
	HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
	HttpResponse::Ok().body("I don't know why this is \"manual\"")
}

#[post("/detect")]
async fn detect(path: String) -> impl Responder {


	HttpResponse::Ok().body(detect_file(&path))

}


fn detect_file(path : &str) -> String{

	let mut to_return = String::new();

	
	let contents = fs::read_to_string(path).expect("Idk what you gave me I can't find this file mate...\n");


	println!("These are the raw contents of the file: \n{}", contents);


	//I'll just check if its a pdf initially and then I will setup the api

	
	let magic = &contents[..4];


	if magic == "%PDF"{

		to_return = "Yes this is a pdf file get him!".to_string();



	}
	else {

		to_return = "Safe".to_string();

	}
	
	to_return

}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
	HttpServer::new(|| {
		App::new()
			.service(hello)
			.service(echo)
			.service(detect)
			.route("/hey", web::get().to(manual_hello))
})
.bind(("127.0.0.1",8080))?
.run()
.await
}


	
	


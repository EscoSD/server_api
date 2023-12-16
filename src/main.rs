#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::http::uri::Origin;

const HEALTH_PREFIX: Origin<'static> = uri!("/health");
const TEST_PREFIX: Origin<'static> = uri!("/test");

#[get("/")]
fn health() -> Status {
	Status::Ok
}

#[get("/?<param>")]
fn test(param: &str) -> String {
	println!("{param}");
	String::from(param)
}

#[get("/")]
fn index() -> String {
	String::from("El puto Pable")
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/", routes![index])
		.mount(HEALTH_PREFIX, routes![health])
		.mount(TEST_PREFIX, routes![test])
}


/*#[derive(Serialize, Deserialize)]
struct Maka {
	age: i32,
	name: String,
}

#[get("/", format = "json")]
fn index() -> Json<Maka> {
	Json(Maka {
		age: 23,
		name: String::from("Caracol")
	})
}

#[derive(Serialize)]
struct Maka {
	name: String,
	age: u8
}


#[get("/")]
fn status() -> Redirect {
	Redirect::to(uri!(HEALTH_PREFIX), health())
}*/
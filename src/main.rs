#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::http::uri::Origin;
use rocket::response::Redirect;

const HEALTH_PREFIX: Origin<'static> = uri!("/health");
const TEST_PREFIX: Origin<'static> = uri!("/test");
const GREETING_PREFIX: Origin<'static> = uri!("/greeting");

#[get("/")]
fn health() -> Status {
	Status::Ok
}

#[get("/?<param>")]
fn test(param: &str) -> String {
	println!("{param}");
	String::from(param)
}

#[get("/<name>")]
fn greeting(name: &str) -> String {
	return match name {
		"esco" | "Esco" | "Escolástico" | "escolástico"
		| "escolastico" | "Escolastico" => format!("Que maquinote el {name}"),
		_ => format!("Que pedazo de tolai el {name}")
	};
}

#[get("/")]
fn index() -> Redirect {
	Redirect::to(HEALTH_PREFIX)
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/", routes![index])
		.mount(HEALTH_PREFIX, routes![health])
		.mount(TEST_PREFIX, routes![test])
		.mount(GREETING_PREFIX, routes![greeting])
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
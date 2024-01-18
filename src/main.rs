use rocket::{
    serde::{
        Deserialize, json::Json
    }, 
    get,
    post, 
    build, 
    launch,
    routes
};

#[derive(Deserialize)]
pub struct Task<'r> {
    pub description: &'r str,
    pub complete: bool,
    pub points: i32
}

#[get("/")]
fn index() -> &'static str {
    "Hello, from Rocket!"
}

#[post("/add", data = "<task>", format = "json")]
fn add_task(task: Json<Task<'_>>) {
    println!("{}", task.description);
}

#[launch]
fn rocket() -> _ {
    build()
        .mount("/", routes![index, add_task])
}

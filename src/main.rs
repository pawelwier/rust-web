use std::{
    sync::atomic::{
        AtomicUsize, Ordering
    }, 
    io::Error,
    fs::File
};
use serde_json::value::{Map, Value};
use handlebars::{Handlebars, to_json};
use rocket::{
    serde::{
        Deserialize,
        json::Json
    },
    fs::NamedFile,
    response::status,
    get,
    post, 
    build, 
    launch,
    routes, 
    Responder,
    State
};

#[derive(Deserialize)]
pub struct Task<'r> {
    pub description: &'r str,
    pub complete: bool,
    pub points: i32
}

pub struct HitCount {
    pub count: AtomicUsize
}

#[derive(Responder)]
#[response(status = 222, content_type = "json")]
pub struct TaskResponseJson(String);

#[get("/")]
fn index(hit_count: &State<HitCount>) -> String {
    let current_hit_count = hit_count.count.fetch_add(1, Ordering::Relaxed) + 1;
    format!("Hello, from Rocket! Hits: {}", current_hit_count)
}

#[get("/home")]
async fn home() -> Result<NamedFile, Error> {
    NamedFile::open("target/index.html").await
}

#[get("/hits")]
fn get_hit_count(hit_count: &State<HitCount>) -> String {
    let current_hit_count = hit_count.count.load(Ordering::Relaxed);
    format!("Hits: {}", current_hit_count)
}

#[post("/add/<id>", data = "<task>", format = "json")]
fn add_task(id: usize, task: Json<Task<'_>>) -> status::Accepted<String> {
    println!("{}", task.description);
    status::Accepted(format!("id: {}", id))
}

#[post("/add-task", data="<task>", format="json")]
fn add_task_res(task: Json<Task<'_>>) -> TaskResponseJson {
    let complete = if task.complete { "yes" } else { "no" };
    let response_str: String = format!(
        "{{\"name\": \"{}\",\"name\": \"{}\",\"points\": \"{}\"}}", 
        task.description, complete, task.points
    );
    TaskResponseJson(response_str)
}

// TODO: move to fn, set to hit count and display
fn make_data() -> Map<String, Value> {
    let mut data = Map::new();
    data.insert("hit_count".to_string(), to_json("two"));
    data
}

#[launch]
fn rocket() -> _ {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("home", "src/templates/index.hbs").unwrap();

    let data = make_data();

    let output_file = File::create("target/index.html");

    handlebars.render_to_write("home", &data, &output_file.unwrap());

    build()
        .manage(HitCount { count: AtomicUsize::new(0) })
        .mount("/", routes![
            home, index, add_task, add_task_res, get_hit_count
        ])
    
}

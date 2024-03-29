use std::{
    sync::atomic::{
        AtomicUsize, Ordering
    }, 
    io::{Error, Write},
    fs::File
};
use serde_json::value::{Map, Value};
use handlebars::{Handlebars, to_json};
use rocket::{
    serde::{
        Deserialize,
        json::Json
    },
    fs::{
        NamedFile,
        FileServer
    },
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

fn update_index_file(current_hit_count: usize) {
    let mut handlebars = Handlebars::new();
    let handlebar_file = handlebars.register_template_file("home", "target/t1.hbs");

    match handlebar_file {
        Ok(_) => {
            let data = make_data(current_hit_count);
            let output_file = File::create("target/index.html");
        
            match output_file {
                Ok(file) => {
                    let _ = handlebars.render_to_write("home", &data, &file);
                }
                Err(e) => { println!("Write to file error: {}", e) }
            }
        }
        Err(e) => { println!("Read template file file error: {}", e) }
    }

}

fn increment_hit_count(hit_count: &State<HitCount>) -> usize {
    let count = hit_count.count.fetch_add(1, Ordering::Relaxed) + 1;
    update_index_file(count);
    count
}

#[get("/count")]
fn index(hit_count: &State<HitCount>) -> String {
    let current_hit_count = increment_hit_count(&hit_count);
    format!("Hello, from Rocket! Hits: {}", current_hit_count)
}

#[get("/")]
async fn home(hit_count: &State<HitCount>) -> Result<NamedFile, Error> {
    let _ = increment_hit_count(&hit_count);
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

fn make_data(count: usize) -> Map<String, Value> {
    let mut data = Map::new();
    data.insert("hit_count".to_string(), to_json(count));
    data
}



#[launch]
fn rocket() -> _ {
    let t1_hbs = "
        <!DOCTYPE html>
        <html lang=\"en\">
        <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>Rocket Web | Index</title>
        </head>
        <body>
        <div>
            Page visits: {{ hit_count }}
        </div>
        </body>
        </html>
    ";
    let mut template_file = File::create("target/t1.hbs").unwrap();
    template_file.write(t1_hbs.as_bytes()).unwrap();
    build()
        .manage(HitCount { count: AtomicUsize::new(0) })
        .mount("/", routes![
            home, index, add_task, add_task_res, get_hit_count
        ])
        // .mount("/file", FileServer::from("static"))
    
}

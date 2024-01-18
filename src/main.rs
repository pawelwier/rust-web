use rocket::{
    serde::{
        Deserialize, json::Json
    }, 
    response::status,
    get,
    post, 
    build, 
    launch,
    routes, 
    Responder
};

#[derive(Deserialize)]
pub struct Task<'r> {
    pub description: &'r str,
    pub complete: bool,
    pub points: i32
}

#[derive(Responder)]
#[response(status = 222, content_type = "json")]
pub struct TaskResponseJson(String);

#[get("/")]
fn index() -> &'static str {
    "Hello, from Rocket!"
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

#[launch]
fn rocket() -> _ {
    build()
        .mount("/", routes![
            index, add_task, add_task_res
        ])
}

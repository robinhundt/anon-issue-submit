#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

use std::env;

use rocket::{
    self, get, post,
    response::status::{BadRequest, Created},
    routes, State,
};
use rocket_contrib::{json::Json, serve::StaticFiles};

use rocket_cors::{AllowedOrigins, CorsOptions};

use serde_derive::{Deserialize, Serialize};

use gitlab::{Gitlab, Issue, ProjectId, UserBasic};

use dotenv::dotenv;

#[derive(Debug)]
struct App {
    gitlab: Gitlab,
    user: UserBasic,
    project_id: ProjectId,
}

#[derive(Debug, Clone, Deserialize)]
struct CreateIssue {
    title: String,
    description: String,
    confidential: bool,
}

#[post("/issue", format = "json", data = "<issue_data>")]
fn issue_create(
    issue_data: Json<CreateIssue>,
    app: State<App>,
) -> Result<Created<Json<Issue>>, BadRequest<String>> {
    let issue_data = issue_data.into_inner();
    let issue = Issue::new(app.project_id, issue_data.title, app.user.clone())
        .with_confidential(issue_data.confidential)
        .with_description(issue_data.description);
    let created_issue = app
        .gitlab
        .create_issue(app.project_id, issue)
        .map_err(|err| BadRequest(Some(format!("Failed to create Issue: {}", err))))?;
    Ok(Created(String::from(""), Some(Json(created_issue))))
}

fn main() {
    dotenv().ok();
    let host = env::var("GITLAB_HOST").expect("GITLAB_HOST variable not set!");
    let api_token = env::var("GITLAB_API_TOKEN").expect("GITLAB_API_TOKEN variable not set!");
    let project_id = env::var("ISSUE_PROJECT_ID").expect("ISSUE_PROJECT_ID variable not set!");

    let gitlab = Gitlab::new(host.clone(), api_token)
        .expect(&format!("Unable to connect to instance: {}", host));
    let user = gitlab
        .current_user()
        .expect("Unable to get user from Gitlab instance")
        .into();
    let project_id = ProjectId::new(project_id.parse().expect("Invalid ProjectId"));
    let app = App {
        gitlab,
        user,
        project_id,
    };

    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8080",
        "http://127.0.0.1:8080",
        "http://localhost:8000",
        "http://0.0.0.0:8000",
        "ws://localhost:8080",
    ]);

    let cors = CorsOptions {
        allowed_origins,
        ..Default::default()
    }
    .to_cors()
    .expect("Unable to create Cors struct");

    rocket::ignite()
        .manage(app)
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/frontend/dist")),
        )
        .mount("/api", routes![issue_create])
        .attach(cors)
        .launch();
}

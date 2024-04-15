use axum::{
    routing::get,
    Router,
    response::{Json, Html},
    Form,
};
use serde_json::{Value, json};
use serde::Deserialize;
use askama::Template;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello_text))
        .route("/json", get(hello_json))
        .route("/html", get(hello_html))
        .route("/form", get(render_form).post(handle_form_submit))
        .route("/some_path",  get(hello_template))
        ;

    println!("Serving on http://localhost:3000 ...");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


// `&'static str` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
async fn hello_text() -> &'static str {
    "Hello, World!"
}


// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn hello_json() -> Json<Value> {
    Json(json!({ "domain": "www.sunzhongwei.com", "since": 1573 }))
}


// `Html` will get a `text/html` content-type
async fn hello_html() -> Html<&'static str> {
    Html("
        <h1>Hello HTML</h1>
        <p>Hello, World!</p>
    ")
}

async fn render_form() -> Html<&'static str> {
    Html(r#"
        <html>
        <head>
            <title>Form Example</title>
        </head>
        <body>
            <h1>Form Example</h1>
            <form method="post">
                <label for="field1">Field 1:</label>
                <input type="text" name="field1" id="field1"><br>

                <label for="field2">Field 2:</label>
                <input type="text" name="field2" id="field2"><br>

                <input type="submit" value="Submit">
            </form>
        </body>
        </html>
    "#)
}

#[derive(Deserialize)]
struct FormData {
    field1: String,
    field2: String,
}

async fn handle_form_submit(Form(form_data): Form<FormData>) -> Html<String> {
    let response_html = format!(
        r#"
        <html>
        <head>
            <title>Form Submission Result</title>
        </head>
        <body>
            <h1>Form "Submission" Result</h1>
            <p>Field 1: {}</p>
            <p>Field 2: {}</p>
        </body>
        </html>
    "#,
        form_data.field1, form_data.field2
    );

    Html(response_html)
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

async fn hello_template() -> HelloTemplate<'static> {
    HelloTemplate { name: "world" }
}
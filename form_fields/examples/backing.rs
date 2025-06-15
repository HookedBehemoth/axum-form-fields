use std::{collections::HashMap, sync::Arc};

use axum::{
    Router,
    body::Body,
    extract::{Query, State},
    http::Method,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use form_fields::from_form::FromForm;
use form_fields_macro::FromForm;
use maud::html;
use tokio::{net::TcpListener, sync::Mutex};

// poor immitation of a shared database
// using a HashMap wrapped in a Mutex for simplicity
#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<HashMap<i32, Test>>>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            db: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    async fn add(&self, test: Test) -> i32 {
        let mut db = self.db.lock().await;
        let id = db.len() as i32 + 1;
        db.insert(id, test);
        id
    }
    async fn get(&self, id: i32) -> Option<Test> {
        let db = self.db.lock().await;
        db.get(&id).cloned()
    }
    async fn update(&self, id: i32, test: Test) {
        let mut db = self.db.lock().await;
        db.insert(id, test);
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let state = AppState::new();
    let app = Router::new()
        .route("/", get(backing))
        .route("/", post(backing))
        .with_state(state);

    let listen_addr = "localhost:8080";
    println!("Listening on http://{}", listen_addr);

    let listener = TcpListener::bind(listen_addr).await?;
    axum::serve(listener, app).await
}

#[derive(serde::Deserialize)]
struct FromDb {
    pub id: Option<i32>,
}

async fn backing(
    method: Method,
    State(state): State<AppState>,
    Query(from_db): Query<FromDb>,
    FromForm(mut form): FromForm<Test>,
) -> Response<Body> {
    // Load existing data if editing an entry
    if method == Method::GET {
        if let Some(id) = from_db.id {
            let test = state.get(id).await.unwrap();
            form.load(test);
        }
    }
    // Handle form submission
    if method == Method::POST {
        if let Some(inner) = form.inner() {
            println!("Form submitted: {:?}", inner);
            if let Some(id) = from_db.id {
                state.update(id, inner).await;
            } else {
                let id = state.add(inner).await;
                let url = format!("/?id={}", id);
                return Redirect::to(&url).into_response();
            }
        }
    }

    html! {
        h1 { "Backing Example" }
        @if let Some(id) = from_db.id {
            p { "Editing entry with ID: " (id) }
            a href="/" { "Create new entry" }
        } @else {
            p { "Creating a new entry" }
        }
        form method="POST" {
            (form.text)
            input type="submit";
        }
    }
    .into_response()
}

#[derive(Debug, Clone, FromForm)]
struct Test {
    #[text_field(display_name = "Required Text", max_length = 50)]
    pub text: String,
}

use std::path::PathBuf;

use crate::models::*;
use axum::{extract::Path, routing::get, Json, Router};
use blog::*;

async fn handle_blog(Path(file): Path<String>) -> Result<Json<Post>, String> {
    let file = PathBuf::from_iter(&["..", "blog-md", &format!("{file}.md")]);
    //.expect("unable to read ../blog-md/<file>");

    let p = Post::from_file(&file);

    Ok(Json(p))
}

async fn handle_posts() -> Json<Vec<Post>> {
    let dir = std::fs::read_dir(PathBuf::from_iter(&["..", "blog-md"]))
        .expect("unable to read ../blog-md");

    // This is truly horrifying, but I'm half awake
    Json(
        dir.filter_map(|f| {
            f.ok()
                .and_then(|f| {
                    f.path()
                        .extension()
                        .filter(|n| n == &"md")
                        .map(|_| f.path())
                })
                .map(|f| Post::from_file(&f))
        })
        .collect(),
    )
}

pub fn router() -> Router {
    Router::new()
        .route("/posts", get(handle_posts))
        .route("/:slug", get(handle_blog))
}

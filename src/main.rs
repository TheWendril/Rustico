#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use rocket_dyn_templates::{Template, context};
use chrono::NaiveDateTime;
use deunicode::deunicode;
use rocket::fs::{FileServer, relative};
use rocket_cors::{AllowedOrigins, CorsOptions};



type Db = Pool<Sqlite>;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct Post {
    id: i64,
    title: String,
    slug: String,
    author: String,
    content: String,
    created_at: NaiveDateTime,
}

#[derive(FromForm)]
struct NewPost {
    title: String,
    author: String,
    content: String
}

/* ===========================
   ROTAS PÚBLICAS
=========================== */

#[get("/")]
async fn index(db: &State<Db>) -> Template {
    
    let posts = sqlx::query_as::<_, Post>(
        "SELECT * FROM posts ORDER BY id DESC LIMIT 3"
    ).fetch_all(db.inner())
     .await
     .unwrap();

    Template::render("index", context! {
        feed: posts
    })

}

#[get("/artigos")]
async fn articles(db: &State<Db>) -> Template {
    
    let posts = sqlx::query_as::<_, Post>(
        "SELECT * FROM posts ORDER BY id DESC"
    ).fetch_all(db.inner())
     .await
     .unwrap();

    Template::render("artigos", context! {
        feed: posts
    })

}


#[get("/sobre")]
async fn about() -> Template {
    Template::render("sobre", context! {})
}

#[get("/post/<slug>")]
async fn show_post(db: &State<Db>, slug: String) -> Template {

    let post = sqlx::query_as::<_, Post>(
        "SELECT * FROM posts WHERE slug = ?"
    )
     .bind(&slug)
     .fetch_one(db.inner())
     .await
     .unwrap();

    Template::render("post", context! {
        article: post
    })
}

/* ===========================
   ADMIN
=========================== */

#[get("/admin")]
async fn admin() -> RawHtml<String> {
    RawHtml(
        r#"
        <style>
            body {
                font-family: Arial, sans-serif;
                background: #0f0f0f;
                color: white;
                display: flex;
                justify-content: center;
                align-items: center;
                height: 100vh;
                margin: 0;
            }

            .card {
                background: #1c1c1c;
                padding: 3rem;
                border-radius: 12px;
                text-align: center;
                width: 400px;
                box-shadow: 0 10px 40px rgba(0,0,0,0.4);
            }

            .btn {
                display: block;
                margin-top: 1.5rem;
                padding: 0.8rem;
                background: #f74c00;
                color: white;
                text-decoration: none;
                border-radius: 6px;
                font-weight: bold;
            }

            .btn.secondary {
                background: #333;
            }
        </style>

        <div class="card">
            <h1>⚙ Painel Admin</h1>
            <p>Gerencie o conteúdo do Rustico</p>

            <a class="btn" href="/admin/posts/new">+ Novo Post</a>
            <a class="btn secondary" href="/">← Voltar</a>
        </div>
        "#
        .to_string(),
    )
}

#[get("/admin/posts/new")]
async fn new_post_form() -> RawHtml<String> {
    RawHtml(
        r#"
        <style>
            body {
                font-family: Arial, sans-serif;
                background: #0f0f0f;
                color: #fff;
                margin: 0;
                display: flex;
                justify-content: center;
                align-items: center;
                min-height: 100vh;
            }

            .container {
                background: #1c1c1c;
                padding: 3rem;
                border-radius: 12px;
                width: 500px;
                box-shadow: 0 15px 40px rgba(0,0,0,0.4);
            }

            h1 {
                margin-top: 0;
                margin-bottom: 2rem;
                text-align: center;
            }

            form {
                display: flex;
                flex-direction: column;
                gap: 1rem;
            }

            input, textarea {
                background: #2a2a2a;
                border: 1px solid #333;
                color: #fff;
                padding: 0.8rem;
                border-radius: 6px;
                font-size: 1rem;
                outline: none;
                transition: 0.2s ease;
            }

            input:focus, textarea:focus {
                border-color: #f74c00;
            }

            textarea {
                min-height: 180px;
                resize: vertical;
            }

            button {
                margin-top: 1rem;
                padding: 0.9rem;
                border: none;
                border-radius: 6px;
                background: #f74c00;
                color: #fff;
                font-size: 1rem;
                font-weight: bold;
                cursor: pointer;
                transition: 0.2s ease;
            }

            button:hover {
                opacity: 0.85;
            }

            .back-link {
                display: block;
                margin-top: 1.5rem;
                text-align: center;
                text-decoration: none;
                color: #aaa;
                font-size: 0.9rem;
            }

            .back-link:hover {
                color: #fff;
            }
        </style>

        <div class="container">
            <h1> Novo Post</h1>

            <form action="/admin/posts" method="post">
                <input name="title" placeholder="Título" required />
                <input name="author" placeholder="Autor" required />
                <textarea name="content" placeholder="Conteúdo do post..." required></textarea>
                <button type="submit">Salvar Post</button>
            </form>

            <a class="back-link" href="/admin">← Voltar ao painel</a>
        </div>
        "#
        .to_string(),
    )
}

fn slugify(title: &str) -> String {
    deunicode(title)
        .to_lowercase()
        .trim()
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}

#[post("/admin/posts", data = "<form>")]
async fn create_post(db: &State<Db>, form: Form<NewPost>) -> RawHtml<String> {
    
    let slug = slugify(&form.title);

    sqlx::query(
        "INSERT INTO posts (title, slug, content, author) VALUES (?, ?, ?, ?)"
    )
    .bind(&form.title)
    .bind(&slug)
    .bind(&form.content)
    .bind(&form.author)
    .execute(db.inner())
    .await
    .unwrap();

    RawHtml(
        r#"
        <style>
            body {
                margin: 0;
                font-family: Arial, sans-serif;
                background: #0f0f0f;
                color: #fff;
                display: flex;
                justify-content: center;
                align-items: center;
                height: 100vh;
            }

            .success-card {
                background: #1c1c1c;
                padding: 2.5rem;
                border-radius: 12px;
                text-align: center;
                width: 400px;
                box-shadow: 0 15px 40px rgba(0,0,0,0.4);
            }

            .success-card h2 {
                margin-top: 0;
                color: #4caf50;
            }

            .btn {
                display: inline-block;
                margin-top: 1.5rem;
                padding: 0.8rem 1.4rem;
                background: #f74c00;
                color: white;
                text-decoration: none;
                border-radius: 6px;
                font-weight: bold;
                transition: 0.2s ease;
            }

            .btn:hover {
                opacity: 0.85;
            }
        </style>

        <div class="success-card">
            <h2>✔ Post criado com sucesso!</h2>
            <p>Seu conteúdo já está disponível no blog.</p>
            <a class="btn" href="/">Voltar para a página inicial</a>
        </div>
        "#
        .to_string(),
    )
}

/* ===========================
   INIT
=========================== */

async fn init_db() -> Db {

    let db = SqlitePoolOptions::new()
        .connect("sqlite://blog.db")
        .await
        .expect("Erro ao conectar no banco");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            slug TEXT NOT NULL UNIQUE,
            content TEXT NOT NULL,
            author TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#
    )
    .execute(&db)
    .await
    .unwrap();

    db
}

#[launch]
async fn rocket() -> _ {
    let db = init_db().await;

    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(), 
        allowed_methods: vec!["GET".parse().unwrap(), "POST".parse().unwrap(), "PUT".parse().unwrap(), "DELETE".parse().unwrap()].into_iter().collect(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Erro ao configurar CORS");

    rocket::build()
        .manage(db)
        .attach(Template::fairing())
        .attach(cors)
        .mount("/static", FileServer::from(relative!("static")))
        .mount(
            "/",
            routes![
                index,
                articles,
                about,
                show_post,
                admin,
                new_post_form,
                create_post
            ],
        )
}

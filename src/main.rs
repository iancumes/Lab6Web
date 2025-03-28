use actix_web::{delete, get, patch, post, put, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct NewMatch {
    home_team: String,
    away_team: String,
    match_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Match {
    id: i64,
    home_team: String,
    away_team: String,
    match_date: String,
    goals: i64,
    yellow_cards: i64,
    red_cards: i64,
    extra_time: bool,
}

type DbPool = Pool<Sqlite>;

// La función init_db ya no es estrictamente necesaria si la tabla ya existe, pero la dejamos
// para que en tiempo de ejecución se verifique (en caso de que la base de datos se recree).
async fn init_db(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS matches (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            home_team TEXT NOT NULL,
            away_team TEXT NOT NULL,
            match_date TEXT NOT NULL,
            goals INTEGER NOT NULL DEFAULT 0,
            yellow_cards INTEGER NOT NULL DEFAULT 0,
            red_cards INTEGER NOT NULL DEFAULT 0,
            extra_time BOOLEAN NOT NULL DEFAULT 0
        )
        "#
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[get("/api/matches")]
async fn get_matches(pool: web::Data<DbPool>) -> impl Responder {
    let matches = sqlx::query_as!(
        Match,
        r#"SELECT id, home_team, away_team, match_date, goals, yellow_cards, red_cards, extra_time FROM matches"#
    )
    .fetch_all(pool.get_ref())
    .await;

    match matches {
        Ok(ms) => HttpResponse::Ok().json(ms),
        Err(e) => {
            eprintln!("Error al obtener partidos: {:?}", e);
            HttpResponse::InternalServerError().body("Error al obtener partidos")
        }
    }
}

#[get("/api/matches/{id}")]
async fn get_match(path: web::Path<i64>, pool: web::Data<DbPool>) -> impl Responder {
    let id = path.into_inner();
    let m = sqlx::query_as!(
        Match,
        r#"SELECT id, home_team, away_team, match_date, goals, yellow_cards, red_cards, extra_time FROM matches WHERE id = ?"#,
        id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match m {
        Ok(Some(match_data)) => HttpResponse::Ok().json(match_data),
        Ok(None) => HttpResponse::NotFound().body("Partido no encontrado"),
        Err(e) => {
            eprintln!("Error al obtener partido: {:?}", e);
            HttpResponse::InternalServerError().body("Error al obtener partido")
        }
    }
}

#[post("/api/matches")]
async fn create_match(new_match: web::Json<NewMatch>, pool: web::Data<DbPool>) -> impl Responder {
    let result = sqlx::query!(
        r#"INSERT INTO matches (home_team, away_team, match_date) VALUES (?, ?, ?)"#,
        new_match.home_team,
        new_match.away_team,
        new_match.match_date
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(res) => {
            let id = res.last_insert_rowid();
            HttpResponse::Ok().json(serde_json::json!({ "id": id }))
        }
        Err(e) => {
            eprintln!("Error al crear partido: {:?}", e);
            HttpResponse::InternalServerError().body("Error al crear partido")
        }
    }
}

#[put("/api/matches/{id}")]
async fn update_match(
    path: web::Path<i64>,
    updated_match: web::Json<NewMatch>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query!(
        r#"UPDATE matches SET home_team = ?, away_team = ?, match_date = ? WHERE id = ?"#,
        updated_match.home_team,
        updated_match.away_team,
        updated_match.match_date,
        id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Partido actualizado"),
        Err(e) => {
            eprintln!("Error al actualizar partido: {:?}", e);
            HttpResponse::InternalServerError().body("Error al actualizar partido")
        }
    }
}

#[delete("/api/matches/{id}")]
async fn delete_match(path: web::Path<i64>, pool: web::Data<DbPool>) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query!(r#"DELETE FROM matches WHERE id = ?"#, id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Partido eliminado"),
        Err(e) => {
            eprintln!("Error al eliminar partido: {:?}", e);
            HttpResponse::InternalServerError().body("Error al eliminar partido")
        }
    }
}

#[patch("/api/matches/{id}/goals")]
async fn register_goal(path: web::Path<i64>, pool: web::Data<DbPool>) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query!(
        r#"UPDATE matches SET goals = goals + 1 WHERE id = ?"#,
        id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Gol registrado correctamente"),
        Err(e) => {
            eprintln!("Error al registrar gol: {:?}", e);
            HttpResponse::InternalServerError().body("Error al registrar gol")
        }
    }
}

#[patch("/api/matches/{id}/yellowcards")]
async fn register_yellow_card(path: web::Path<i64>, pool: web::Data<DbPool>) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query!(
        r#"UPDATE matches SET yellow_cards = yellow_cards + 1 WHERE id = ?"#,
        id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Tarjeta amarilla registrada correctamente"),
        Err(e) => {
            eprintln!("Error al registrar tarjeta amarilla: {:?}", e);
            HttpResponse::InternalServerError().body("Error al registrar tarjeta amarilla")
        }
    }
}

#[patch("/api/matches/{id}/redcards")]
async fn register_red_card(path: web::Path<i64>, pool: web::Data<DbPool>) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query!(
        r#"UPDATE matches SET red_cards = red_cards + 1 WHERE id = ?"#,
        id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Tarjeta roja registrada correctamente"),
        Err(e) => {
            eprintln!("Error al registrar tarjeta roja: {:?}", e);
            HttpResponse::InternalServerError().body("Error al registrar tarjeta roja")
        }
    }
}

#[patch("/api/matches/{id}/extratime")]
async fn set_extra_time(path: web::Path<i64>, pool: web::Data<DbPool>) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query!(
        r#"UPDATE matches SET extra_time = 1 WHERE id = ?"#,
        id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Tiempo extra establecido correctamente"),
        Err(e) => {
            eprintln!("Error al establecer tiempo extra: {:?}", e);
            HttpResponse::InternalServerError().body("Error al establecer tiempo extra")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no definida en .env");
    let pool = SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Error conectando a la base de datos");

    // Llama a init_db por si la base de datos se recrea en algún momento.
    init_db(&pool)
        .await
        .expect("Error inicializando la base de datos");

    println!("Servidor escuchando en el puerto 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_matches)
            .service(get_match)
            .service(create_match)
            .service(update_match)
            .service(delete_match)
            .service(register_goal)
            .service(register_yellow_card)
            .service(register_red_card)
            .service(set_extra_time)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

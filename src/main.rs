use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
// use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub id: i64,
    pub name: String,
    pub power: i32,
    pub leader: i32,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct DraftData {
    pub leader_name: String,
    pub member_name: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct DraftCount {
    pub count: i32,
}

#[get("/api/player")]
async fn get_member() -> impl Responder {
    let db_pool = connect_db().await;
    let result = sqlx::query_as::<_, Player>(r#"SELECT id, name, power, leader FROM players"#)
        .fetch_all(db_pool.get_ref())
        .await;

    match result {
        Ok(players) => {
            println!("{:?}", players);
            HttpResponse::Ok().json(players)
        }
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    }
}

#[post("/api/register")]
async fn post_register(player_data: web::Json<Player>) -> impl Responder {
    // データベースへの接続
    let db_pool = connect_db().await;
    let conn = db_pool.acquire().await;
    let player = player_data.into_inner();
    let player_return = player.clone();
    println!("{:?}", player);
    let result = sqlx::query!(
        r#"
        INSERT INTO players (id,name,power,leader) VALUES (?,?,?,?)
        "#,
        player.id,
        player.name,
        player.power,
        player.leader
    )
    .execute(&mut *conn.unwrap())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(player_return),
        Err(error) => {
            println!("Error to try INSERT");
            println!("{}", error.to_string());
            HttpResponse::BadRequest().body("Eror trying to create new user")
        }
    }
}

#[post("/api/form")]
async fn post_draft_form(draft_data: web::Json<DraftData>) -> impl Responder {
    // データベースへの接続
    let db_pool = connect_db().await;
    let conn = db_pool.acquire().await;
    let draft = draft_data.into_inner();
    let draft_return = draft.clone();
    println!("{:?}", draft);

    // データベースへの格納
    let result = sqlx::query!(
        r#"
        INSERT INTO draft (leader,member) VALUES (?,?)
        "#,
        draft.leader_name,
        draft.member_name
    )
    .execute(&mut *conn.unwrap())
    .await;
    match result {
        Ok(_) => HttpResponse::Ok().json(draft_return.clone()),
        Err(error) => {
            println!("Error to try INSERT");
            println!("{}", error.to_string());
            return HttpResponse::BadRequest().body("Eror trying to post draft result");
        }
    };

    // データベースに格納されたデータの数を確認する
    let count_result = sqlx::query_as::<_, DraftCount>(r#"select count(*) from draft"#)
        .fetch_one(db_pool.get_ref())
        .await;

    let mut count_num = 0;
    match count_result {
        Ok(num) => count_num = num.count,
        Err(error) => {
            println!("Error to try Count");
            println!("{}", error.to_string());
            return HttpResponse::BadRequest().body("Eror trying to count draft result");
        }
    };

    // もし格納されているデータ数がチーム数と一致したら、今までのドラフトデータを開示
    // とりあえず今は4に固定
    if count_num == 4 {
        // データベースからドラフトの一覧を配列で取り出す
        todo!();
        // draftテーブルを初期化する
        todo!();
        HttpResponse::Ok().json(draft_return)
    } else {
        // 何もせずに終了
        HttpResponse::Ok().json(draft_return)
    }
}

#[delete("/api/init")]
async fn initialize_db() -> impl Responder {
    let db_pool = connect_db().await;
    let conn = db_pool.acquire().await;
    let result = sqlx::query!(
        r#"
            DELETE FROM players
        "#
    )
    .execute(&mut *conn.unwrap())
    .await;
    match result {
        Ok(_) => HttpResponse::Created().body("Delete complete"),
        _ => HttpResponse::BadRequest().body("Error trying to init"),
    }
}

async fn connect_db() -> web::Data<SqlitePool> {
    let database_url = "sqlite:./database.db";

    web::Data::new(
        SqlitePool::connect(&database_url)
            .await
            .expect("Failed to create DB pool"),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_member)
            .service(post_register)
            .service(initialize_db)
            .service(post_draft_form)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

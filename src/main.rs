// https://javierviola.com/post/03-basic-crud-with-rust-using-tide-move-to-db/
// https://github.com/pepoviola/tide-basic-crud

// https://github.com/launchbadge/sqlx
// https://github.com/eaze/tide-sqlx
// https://docs.rs/sqlx-core/0.5.5/sqlx_core/pool/struct.Pool.html
// https://github.com/NyxCode/ormx/blob/master/example-postgres/src/main.rs

// https://www.lpalmieri.com/posts/error-handling-rust/
// https://blog.logrocket.com/11-database-drivers-and-orms-for-rust-that-are-ready-for-production/
// https://appsody.dev/tutorials/ServerlessRust/
// https://gruberbastian.com/posts/deploy_rust/
// https://www.lpalmieri.com/posts/2020-07-04-choosing-a-rust-web-framework-2020-edition/

use tide::{Request, Response, Body};
use serde::{Serialize, Deserialize};
use sqlx::{Any, Pool};
use sqlx::any::AnyPoolOptions;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
struct User { name: String, email: String }

#[derive(Clone)]
struct State {
    pool: Pool<Any>,
}

// docker run -p 8080:8080 --link pg:pg --name crud -d rust-tide-sqlx-crud

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    sqlx::any::install_default_drivers();

    // docker run --name pg -e POSTGRES_PASSWORD=changeit -d -v ~/schema:/var/lib/postgresql/data -p 5432:5432 postgres
    // docker run -it --rm --link pg:pg postgres psql -h pg -U postgres

    let pool = AnyPoolOptions::new()
        .max_connections(5)
        //.connect("postgres://postgres:changeit@pg/postgres")
        .connect("sqlite://target/test.db?mode=rwc")
        .await
        .expect("db is ok");

    sqlx::migrate!("./db")
        .run(&pool)
        .await
        .expect("db is migrated");

    tide::log::start();
    let mut app = tide::with_state(State { pool });

    // Create
    app.at("/users").post(|mut req: Request<State>| async move {
        let user: User = req.body_json().await?;
        println!("user: {:?}", user);

        let pool = req.state().pool.clone();
        sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
            .bind(user.name)
            .bind(user.email)
            .execute(&pool)
            .await?;

        Ok(Response::new(201))
    });

    // Read
    app.at("/users").get(|req: Request<State>| async move {
        let pool = req.state().pool.clone();

        let users = sqlx::query_as::<_, User>("SELECT name, email FROM users")
            .fetch_all(&pool)
            .await?;

        let mut res = Response::new(200);
        res.set_body(Body::from_json(&users)?);
        Ok(res)
    });

    // Update
    app.at("/users/:name").put(|mut req: Request<State>| async move {
        let name = req.param("name")?.to_owned();
        let email = req.body_string().await?;

        let pool = req.state().pool.clone();
        sqlx::query("UPDATE users SET email = $2 WHERE name = $1")
            .bind(name)
            .bind(email)
            .execute(&pool)
            .await?;

        Ok(Response::new(204))
    });

    // Delete
    app.at("/users/:name").delete(|req: Request<State>| async move {
        let name = req.param("name")?.to_owned();

        let pool = req.state().pool.clone();
        sqlx::query("DELETE FROM users WHERE name = $1")
            .bind(name)
            .execute(&pool)
            .await?;

        Ok(Response::new(204))
    });

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

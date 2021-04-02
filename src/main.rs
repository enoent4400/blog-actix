use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set.");

    let app = blog_actix::Blog::new(8998);

    app.run(database_url)
}

mod http; // declare http as a submodule of main (crate module)
mod server; // declare server as a submodule of main (crate module)

use http::Method; // from http, use the Method enum (exposed via "pub use..." in http/mod.rs)
use http::Request; // from http, use the Request struct (exposed via "pub use..." in http/mod.rs)

use std::env;
use dotenv::dotenv;

macro_rules! hw {
    () => {
        println!("Hello world!");
    };
}

fn main() {
    dotenv().ok(); // ensure .env can be loaded

    let database_url = env::var("DATABASE_URL").expect("You've not set the DATABASE_URL");

    hw!();

    // Print the value
    println!("Database URL: {}", database_url);

    let server = server::Server::new(String::from("127.0.0.1:8080"));
    server.run();
}
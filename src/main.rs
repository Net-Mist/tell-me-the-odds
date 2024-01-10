use core::panic;

use millennium_falcon::infrastructure_service::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match run("127.0.0.1:8000") {
        Ok(server) => server.await,
        Err(e) => {
            let e = e.context("unable to start the server");
            println!("{e:#?}");
            panic!();
        }
    }
}

extern crate getopts;

use actix_web::{middleware, web, App, HttpServer};
use env_logger;
use getopts::Options;
use std::env;

mod notes;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("l", "listen", "specify the address to listen on", "ADDR");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let addr = match matches.opt_str("l") {
        Some(m) => m,
        None => String::from("0.0.0.0:13999"),
    };
    let db = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Database Initialize
    let client = redis::Client::open(db.as_str()).unwrap();

    start(addr, client)
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} DB_STR [options]", program);
    print!("{}", opts.usage(&brief));
}

fn start(addr: String, client: redis::Client) {
    HttpServer::new(move || {
        App::new()
            .data(client.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/notes")
                    .route("/", web::post().to(notes::put_note))
                    .route("/{id}", web::get().to(notes::get_note)),
            )
    })
    .bind(addr)
    .unwrap()
    .run()
    .unwrap();
}

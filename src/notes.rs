extern crate rand;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use redis::Commands;

#[derive(Deserialize, Serialize)]
pub struct Note {
    data: String,
}

#[derive(Serialize)]
pub struct NoteResponse {
    id: String,
}

fn generate_id() -> String {
    let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    rand_string
}

pub fn put_note(note: web::Json<Note>, db: web::Data<redis::Client>) -> HttpResponse {
    let id = generate_id();
    let mut con = match db.get_connection() {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::InternalServerError().finish();
        }
    };
    let _: () = con.set(&id, &note.data).unwrap();
    HttpResponse::Ok().json(NoteResponse { id: id })
}

pub fn get_note(path: web::Path<String>, db: web::Data<redis::Client>) -> HttpResponse {
    let mut con = match db.get_connection() {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::InternalServerError().finish();
        }
    };
    let data = match con.get(path.clone()) {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::NotFound().finish();
        }
    };
    let note = Note { data: data };
    HttpResponse::Ok().json(note)
}

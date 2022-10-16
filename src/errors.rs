use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};

#[derive(Debug)]
pub enum Error {
    DatabaseError(diesel::result::Error),
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match self {
            Error::DatabaseError(e) => match e {
                diesel::result::Error::NotFound => Status::NotFound.respond_to(req),
                // TODO: log the error but dont send to client
                _ => (Status::InternalServerError, e.to_string()).respond_to(req),
            },
        }
    }
}

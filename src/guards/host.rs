// Source: https://github.com/SergioBenitez/Rocket/issues/823#issuecomment-437646743
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;

pub(crate) struct HostHeader(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HostHeader {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Host") {
            Some(h) => Outcome::Success(HostHeader(h.to_string())),
            None => Outcome::Forward(()),
        }
    }
}

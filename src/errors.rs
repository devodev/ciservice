use rocket::http::Status;
use rocket::request::Request;
use rocket::response::status;
use rocket::response::{self, Responder};
use rocket::serde::json::{json, Json};
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug)]
pub enum Errors {
    ValidationErrors(ValidationErrors),
    DatabaseError(diesel::result::Error),
}

impl<'r> Responder<'r, 'static> for Errors {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match self {
            Errors::ValidationErrors(err) => {
                use validator::ValidationErrorsKind::Field;

                let mut errors = json!({});
                for (field, field_errors) in err.into_errors() {
                    if let Field(field_errors) = field_errors {
                        errors[field] = field_errors
                            .into_iter()
                            .map(|field_error| field_error.code)
                            .collect();
                    }
                }

                status::Custom(
                    Status::UnprocessableEntity,
                    Json(json!({ "errors": errors })),
                )
                .respond_to(req)
            }
            Errors::DatabaseError(e) => match e {
                diesel::result::Error::NotFound => Status::NotFound.respond_to(req),
                // TODO: log the error but dont send to client
                _ => (Status::InternalServerError, e.to_string()).respond_to(req),
            },
        }
    }
}

pub struct FieldValidator {
    errors: ValidationErrors,
}

impl Default for FieldValidator {
    fn default() -> Self {
        Self {
            errors: ValidationErrors::new(),
        }
    }
}

impl FieldValidator {
    pub fn validate<T: Validate>(model: &T) -> Self {
        Self {
            errors: model.validate().err().unwrap_or_else(ValidationErrors::new),
        }
    }

    /// Convenience method to trigger early returns with ? operator.
    pub fn check(self) -> Result<(), Errors> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(Errors::ValidationErrors(self.errors))
        }
    }

    pub fn extract<T>(&mut self, field_name: &'static str, field: Option<T>) -> T
    where
        T: Default,
    {
        field.unwrap_or_else(|| {
            self.errors
                .add(field_name, ValidationError::new("can't be blank"));
            T::default()
        })
    }
}

// use std::ops::Deref;

// use validator::{Validate, ValidationError, ValidationErrors};

// struct Validated<T>(T);

// impl<T> Deref for Validated<T> {
//     type Target = T;

//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// impl<'f, T> FromForm<'f> for Validated<T>
// where
//     T: Validate + FromForm<'f, Error = FormParseError<'f>>,
// {
//     // In practice, we'd use a more descriptive error type.
//     type Error = ValidationErrors;

//     fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<Self, Self::Error> {
//         let value = T::from_form(items, strict).map_err(|error| {
//             let mut validation_errors = ValidationErrors::new();
//             match error {
//                 FormParseError::BadValue(name, value) => {
//                     let mut v_error = ValidationError::new("bad_value");
//                     v_error.add_param("name".into(), &name.as_str());
//                     v_error.add_param("value".into(), &value.as_str());
//                     validation_errors.add("rocket-form-field", v_error);
//                 }
//                 FormParseError::Unknown(name, value) => {
//                     let mut v_error = ValidationError::new("unknown_value");
//                     v_error.add_param("name".into(), &name.as_str());
//                     v_error.add_param("value".into(), &value.as_str());
//                     validation_errors.add("rocket-form-field", v_error);
//                 }
//                 FormParseError::Missing(name) => {
//                     let mut v_error = ValidationError::new("missing_value");
//                     v_error.add_param("name".into(), &name.as_str());
//                     validation_errors.add("rocket-form-field", v_error);
//                 }
//             }

//             validation_errors
//         })?;

//         value.validate()?;
//         Ok(Validated(value))
//     }
// }

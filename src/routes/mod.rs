use rocket::fairing::AdHoc;
use rocket::serde::Serialize;

use crate::database::PaginatedParams;

mod job;
mod playground;

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("Add routes", |rocket| async {
        rocket
            .mount("/api/jobs", job::routes())
            .mount("/playground", playground::routes())
    })
}

#[derive(FromForm, Default)]
pub struct Pagination {
    pub limit: Option<i64>,
    pub page: Option<i64>,
}

impl From<Pagination> for PaginatedParams {
    fn from(value: Pagination) -> Self {
        let mut result = Self::default();
        if let Some(limit) = value.limit {
            result.limit = limit;
        }
        if let Some(page) = value.page {
            result.page = page;
        }
        result
    }
}

#[derive(Serialize)]
pub(crate) struct ListResponse<T> {
    results: Vec<T>,
    count: usize,
}

#[derive(Serialize)]
pub(crate) struct ListPaginatedResponse<T> {
    results: Vec<T>,
    limit: i64,
    page: i64,
    count: usize,
    total: i64,
}

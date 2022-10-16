use rocket::fairing::AdHoc;
use serde::Serialize;

use crate::database::PaginatedParams;

pub(crate) mod job;

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("Api routes", |rocket| async {
        rocket.mount(
            "/api",
            routes![job::create, job::list, job::get, job::update, job::delete],
        )
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
    limit: i64,
    page: i64,
    count: usize,
    total: i64,
}

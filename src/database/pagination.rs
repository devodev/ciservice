// inspired by: https://github.com/diesel-rs/diesel/blob/5dc6b2a9e681c2bf0880b9b75c84edd784e8f221/examples/postgres/advanced-blog-cli/src/pagination.rs
use diesel::{
    pg::Pg,
    query_builder::{AstPass, Query, QueryFragment},
    query_dsl::LoadQuery,
    sql_types::BigInt,
    PgConnection, QueryResult, RunQueryDsl,
};

pub trait Paginate: Sized {
    fn paginate(self, page: i64, limit: i64) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i64, limit: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            limit,
            page,
            offset: page * limit,
        }
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    limit: i64,
    page: i64,
    offset: i64,
}

pub struct PaginatedQueryResult<U> {
    pub data: Vec<U>,
    pub limit: i64,
    pub page: i64,
    pub total: i64,
}

impl<T> Paginated<T> {
    pub fn load_and_count<U>(self, conn: &mut PgConnection) -> QueryResult<PaginatedQueryResult<U>>
    where
        Self: LoadQuery<PgConnection, (U, i64)>,
    {
        let limit = self.limit;
        let page = self.page;

        let results = self.load::<(U, i64)>(conn)?;

        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();

        Ok(PaginatedQueryResult {
            data: records,
            limit,
            page,
            total,
        })
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut pass: AstPass<Pg>) -> QueryResult<()> {
        // this can get expensive if run on big tables
        // consider adding a limit of rows to the count statement
        // or offer the option of not including the total when using
        // pagination.
        pass.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(pass.reborrow())?;
        pass.push_sql(") t LIMIT ");
        pass.push_bind_param::<BigInt, _>(&self.limit)?;
        pass.push_sql(" OFFSET ");
        pass.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}

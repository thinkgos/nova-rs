use sea_orm::{
    ActiveValue, ConnectionTrait, DbErr, DeleteResult, EntityTrait, Order, PaginatorTrait,
    QueryOrder, QuerySelect,
};

use crate::model::log_login::{Column, Entity, Model};

struct LogLoginDal<'a, C: ConnectionTrait>(&'a C);

impl<'a, C: ConnectionTrait> LogLoginDal<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        Self(conn)
    }
    pub async fn delete(&self, id: i64) -> Result<DeleteResult, DbErr> {
        Entity::delete_by_id(id).exec(self.0).await
    }
    pub async fn delete_by_filter(&self, filter: ()) -> Result<DeleteResult, DbErr> {
        Entity::delete_many().exec(self.0).await
    }
    pub async fn get(&self, id: i64) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(self.0).await
    }
    pub async fn get_by_filter(&self, filter: ()) -> Result<Option<Model>, DbErr> {
        Entity::find().one(self.0).await
    }
    pub async fn pluck_id_by_filter(&self, filter: ()) -> Result<Vec<i64>, DbErr> {
        Entity::find()
            .select_only()
            .column(Column::Id)
            .into_tuple()
            .all(self.0)
            .await
    }
    pub async fn count(&self, filter: ()) -> Result<u64, DbErr> {
        Entity::find().count(self.0).await
    }
    pub async fn list(&self, filter: ()) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .limit(20)
            .offset(0)
            .order_by(Column::CreatedAt, Order::Desc)
            .all(self.0)
            .await
    }
    pub async fn list_page(&self, filter: ()) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .limit(20)
            .offset(0)
            .order_by(Column::CreatedAt, Order::Desc)
            .all(self.0)
            .await
    }
}

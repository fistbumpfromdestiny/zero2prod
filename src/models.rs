use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::subscriptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Subscription {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub subscribed_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::subscriptions)]
pub struct NewSubscription<'a> {
    pub email: &'a str,
    pub name: &'a str,
    pub subscribed_at: SystemTime,
}

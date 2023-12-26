use crate::models::{NewSubscription, Subscription};
use crate::schema::subscriptions;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{extract::Form, Extension};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection, RunQueryDsl, SelectableHelper,
};
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    Extension(db_pool): Extension<Pool<ConnectionManager<PgConnection>>>,
    Form(form): Form<FormData>,
) -> Response {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.email
    );

    let _request_span_guard = request_span.enter();
    let mut connection = db_pool.get().expect("Failed to connect to the database.");
    let query_span = tracing::info_span!("Saving new subscriber details in the database.");

    match diesel::insert_into(subscriptions::table)
        .values(NewSubscription {
            email: &form.email,
            name: &form.name,
            subscribed_at: chrono::Utc::now().into(),
        })
        .returning(Subscription::as_returning())
        .get_result(&mut connection)
    {
        Ok(_) => {
            tracing::info!("Saved new subscriber details in the database.");
            StatusCode::OK.into_response()
        }
        Err(error) => {
            tracing::error!(
                error = %error,
                "Failed to save new subscriber details in the database."
            );

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

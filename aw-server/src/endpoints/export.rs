use std::collections::HashMap;

use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_okapi::openapi;

use aw_models::BucketsExport;

use crate::endpoints::{HttpErrorJson, ServerState};

#[openapi]
#[get("/")]
pub fn buckets_export(state: State<ServerState>) -> Result<Json<BucketsExport>, HttpErrorJson> {
    let datastore = endpoints_get_lock!(state.datastore);
    let mut export = BucketsExport {
        buckets: HashMap::new(),
    };
    let mut buckets = match datastore.get_buckets() {
        Ok(buckets) => buckets,
        Err(err) => return Err(err.into()),
    };
    for (bid, mut bucket) in buckets.drain() {
        bucket.events = Some(match datastore.get_events(&bid, None, None, None) {
            Ok(events) => events,
            Err(err) => return Err(err.into()),
        });
        export.buckets.insert(bid, bucket);
    }

    /*
    Ok(Response::build()
        .status(Status::Ok)
        .header(Header::new(
            "Content-Disposition",
            "attachment; filename=aw-buckets-export.json",
        ))
        .sized_body(Cursor::new(
            serde_json::to_string(&export).expect("Failed to serialize"),
        ))
        .finalize())
    */
    Ok(Json(export))
}

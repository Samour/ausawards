use crate::dto::{
  AwardAlternateId, AwardClassification, UpdateAwardClassificationNoteRequest,
  UpdateAwardClassificatonStatusRequest, UpdateAwardExpiryDate,
};
use crate::filters::AuthenticationFilter;
use crate::services::AwardService;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

pub fn route(
  authentication_filter: &Box<dyn AuthenticationFilter>,
  award_service: Arc<dyn AwardService + Send + Sync>,
) -> BoxedFilter<(impl Reply,)> {
  let as1 = Arc::clone(&award_service);
  let add_alt_id = warp::path!(String / "alternateIds")
    .and(warp::post())
    .and(authentication_filter.permissioned("addAwardAlternateId"))
    .and(warp::body::json())
    .and_then(move |award_id: String, alt_id: AwardAlternateId| {
      let as_local = Arc::clone(&as1);
      async move {
        as_local
          .add_alt_id(&award_id, alt_id)
          .await
          .map(|_| warp::reply())
      }
    });

  let as2 = Arc::clone(&award_service);
  let add_classification = warp::path!(String / "classifications")
    .and(warp::post())
    .and(authentication_filter.permissioned("addAwardClassification"))
    .and(warp::body::json())
    .and_then(
      move |award_id: String, classification: AwardClassification| {
        let as_local = Arc::clone(&as2);
        async move {
          as_local
            .add_classification(&award_id, classification)
            .await
            .map(|_| warp::reply())
        }
      },
    );

  let as3 = Arc::clone(&award_service);
  let update_expiry_date = warp::path!(String / "expired")
    .and(warp::put())
    .and(authentication_filter.permissioned("updateAwardExpiryDate"))
    .and(warp::body::json())
    .and_then(move |award_id: String, expired: UpdateAwardExpiryDate| {
      let as_local = Arc::clone(&as3);
      async move {
        as_local
          .update_expired_at(&award_id, Some(expired.expired_at))
          .await
          .map(|_| warp::reply())
      }
    });

  let as4 = Arc::clone(&award_service);
  let remove_expiry_date = warp::path!(String / "expired")
    .and(warp::delete())
    .and(authentication_filter.permissioned("removeAwardExpiryDate"))
    .and_then(move |award_id: String| {
      let as_local = Arc::clone(&as4);
      async move {
        as_local
          .update_expired_at(&award_id, None)
          .await
          .map(|_| warp::reply())
      }
    });

  let as5 = Arc::clone(&award_service);
  let update_classification_status = warp::path!(String / "classifications" / String / "active")
    .and(warp::put())
    .and(authentication_filter.permissioned("updateAwardClassificationActive"))
    .and(warp::body::json())
    .and_then(
      move |award_id: String,
            classification_id: String,
            active: UpdateAwardClassificatonStatusRequest| {
        let as_local = Arc::clone(&as5);
        async move {
          as_local
            .update_classification_status(&award_id, &classification_id, active.active)
            .await
            .map(|_| warp::reply())
        }
      },
    );

  let update_classification_note = warp::path!(String / "classifications" / String / "note")
    .and(warp::put())
    .and(authentication_filter.permissioned("updateAwardClassificationNote"))
    .and(warp::body::json())
    .and_then(
      move |award_id: String,
            classification_id: String,
            note: UpdateAwardClassificationNoteRequest| {
        let as_local = Arc::clone(&award_service);
        async move {
          as_local
            .update_classification_note(&award_id, &classification_id, &note.note)
            .await
            .map(|_| warp::reply())
        }
      },
    );

  add_alt_id
    .or(add_classification)
    .or(update_expiry_date)
    .or(remove_expiry_date)
    .or(update_classification_status)
    .or(update_classification_note)
    .boxed()
}

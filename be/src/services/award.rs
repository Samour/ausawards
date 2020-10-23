use crate::converters::AwardConverter;
use crate::dto;
use crate::dto::CreateAwardRequest;
use crate::model::{Award, AwardClassification};
use crate::repositories::AwardRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;
use warp::Rejection;

#[async_trait]
pub trait AwardService {
  async fn create_award(&self, award: CreateAwardRequest) -> Result<(), Rejection>;
  async fn add_alt_id(
    &self,
    award_id: &str,
    alt_id: dto::AwardAlternateId,
  ) -> Result<(), Rejection>;
  async fn add_classification(
    &self,
    award_id: &str,
    classification: dto::AwardClassification,
  ) -> Result<(), Rejection>;
  async fn update_expired_at(
    &self,
    award_id: &str,
    expired_at: Option<DateTime<Utc>>,
  ) -> Result<(), Rejection>;
  async fn update_classification_status(
    &self,
    award_id: &str,
    classification_id: &str,
    active: bool,
  ) -> Result<(), Rejection>;
  async fn update_classification_note(
    &self,
    award_id: &str,
    classification_id: &str,
    note: &str,
  ) -> Result<(), Rejection>;
}

pub struct AwardServiceImpl {
  award_converter: Arc<dyn AwardConverter + Send + Sync>,
  award_repository: Arc<dyn AwardRepository + Send + Sync>,
}

impl AwardServiceImpl {
  pub fn new(
    award_converter: Arc<dyn AwardConverter + Send + Sync>,
    award_repository: Arc<dyn AwardRepository + Send + Sync>,
  ) -> AwardServiceImpl {
    AwardServiceImpl {
      award_converter,
      award_repository,
    }
  }

  async fn load_award(&self, award_id: &str) -> Result<Award, Rejection> {
    let award = self.award_repository.find_by_id(award_id).await?;
    match award {
      Some(a) => Ok(a),
      None => {
        log::warn!(
          "Attempted to update Award(id={}), but could not be found",
          award_id
        );
        Err(warp::reject::not_found())
      }
    }
  }

  async fn update_award<F>(&self, award_id: &str, update: F) -> Result<(), Rejection>
  where
    F: FnOnce(&mut Award) -> Result<(), Rejection>,
  {
    let award = self.award_repository.find_by_id(award_id).await?;
    if let None = award {
      log::warn!(
        "Attempted to update Award(id={}), but could not be found",
        award_id
      );
      return Err(warp::reject::not_found());
    }

    let mut award = award.unwrap();
    update(&mut award)?;
    self.award_repository.save(&award).await?;

    Ok(())
  }

  async fn update_classification<F>(
    &self,
    award_id: &str,
    classification_id: &str,
    update: F,
  ) -> Result<(), Rejection>
  where
    F: FnOnce(&mut AwardClassification) -> (),
  {
    self.update_award(award_id, |a| {
      for c in &mut a.classifications {
        if c.id == classification_id {
          update(c);
          log::info!("Classification(awardId={}, id={}) updated", award_id, classification_id);
          return Ok(());
        }
      }

    log::warn!("Attempted to update Classification(awardId={}, id={}); but it could not be found on the award", award_id, classification_id);
    Err(warp::reject::not_found())
    }).await
  }
}

#[async_trait]
impl AwardService for AwardServiceImpl {
  async fn create_award(&self, award: CreateAwardRequest) -> Result<(), Rejection> {
    self
      .award_repository
      .save(&Award {
        id: Uuid::new_v4().to_hyphenated().to_string(),
        external_id: award.external_id,
        name: award.name,
        industry_name: award.industry_name,
        common_rule: award.common_rule,
        alternate_ids: award
          .alternate_ids
          .into_iter()
          .map(|i| self.award_converter.alt_id_to_model(i))
          .collect(),
        operative_date: bson::DateTime::from(award.operative_date),
        expired_date: award.expired_date.map(bson::DateTime::from),
        classifications: award
          .classifications
          .into_iter()
          .map(|c| self.award_converter.classification_to_model(c))
          .collect(),
      })
      .await?;

    Ok(())
  }

  async fn add_alt_id(
    &self,
    award_id: &str,
    alt_id: dto::AwardAlternateId,
  ) -> Result<(), Rejection> {
    self
      .update_award(award_id, |a| {
        a.alternate_ids
          .push(self.award_converter.alt_id_to_model(alt_id));
        log::info!("New alternate ID added to Award(id={})", award_id);
        Ok(())
      })
      .await
  }

  async fn add_classification(
    &self,
    award_id: &str,
    classification: dto::AwardClassification,
  ) -> Result<(), Rejection> {
    self
      .update_award(award_id, |a| {
        a.classifications
          .push(self.award_converter.classification_to_model(classification));
        log::info!("New classification added to Award(id={})", award_id);
        Ok(())
      })
      .await
  }

  async fn update_expired_at(
    &self,
    award_id: &str,
    expired_at: Option<DateTime<Utc>>,
  ) -> Result<(), Rejection> {
    self
      .update_award(award_id, |a| {
        a.expired_date = expired_at.map(bson::DateTime::from);
        if let Some(_) = a.expired_date {
          log::info!("Expired date updated for Award(id={})", award_id);
        } else {
          log::info!("Expired date removed from Award(id={})", award_id);
        }
        Ok(())
      })
      .await
  }

  async fn update_classification_status(
    &self,
    award_id: &str,
    classification_id: &str,
    active: bool,
  ) -> Result<(), Rejection> {
    self
      .update_classification(award_id, classification_id, |c| {
        c.active = active;
        log::info!(
          "Classification(awardId={}, id={}) status updated",
          award_id,
          classification_id
        );
      })
      .await
  }

  async fn update_classification_note(
    &self,
    award_id: &str,
    classification_id: &str,
    note: &str,
  ) -> Result<(), Rejection> {
    self
      .update_classification(award_id, classification_id, |c| {
        c.note = String::from(note);
        log::info!(
          "Classification(awardId={}, id={}) note updated",
          award_id,
          classification_id
        );
      })
      .await
  }
}

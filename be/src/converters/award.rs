use crate::dto;
use crate::model;

pub trait AwardConverter {
  fn alt_id_to_model(&self, dto: dto::AwardAlternateId) -> model::AwardAlternateId;
  fn classification_to_model(&self, dto: dto::AwardClassification) -> model::AwardClassification;
}

pub struct AwardConverterImpl {}

impl AwardConverterImpl {
  pub fn new() -> AwardConverterImpl {
    AwardConverterImpl {}
  }
}

impl AwardConverter for AwardConverterImpl {
  fn alt_id_to_model(&self, dto: dto::AwardAlternateId) -> model::AwardAlternateId {
    model::AwardAlternateId {
      id: dto.id,
      id_type: dto.id_type,
    }
  }
  fn classification_to_model(&self, dto: dto::AwardClassification) -> model::AwardClassification {
    model::AwardClassification {
      id: dto.id,
      title: dto.title,
      active: dto.active,
      note: dto.note,
    }
  }
}

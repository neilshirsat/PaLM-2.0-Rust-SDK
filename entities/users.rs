//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub public_key: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub preferred_name: Option<String>,
    pub email: Option<String>,
    pub email_verified_at: Option<DateTimeWithTimeZone>,
    pub avatar: Option<String>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub last_updated_at: Option<DateTimeWithTimeZone>,
    pub has_onboarded: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::build_history::Entity")]
    BuildHistory,
    #[sea_orm(has_many = "super::eating_preferences::Entity")]
    EatingPreferences,
    #[sea_orm(has_many = "super::inventory::Entity")]
    Inventory,
}

impl Related<super::build_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BuildHistory.def()
    }
}

impl Related<super::eating_preferences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EatingPreferences.def()
    }
}

impl Related<super::inventory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inventory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
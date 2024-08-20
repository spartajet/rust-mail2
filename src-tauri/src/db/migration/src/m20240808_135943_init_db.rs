use sea_orm_migration::{prelude::*, schema::*};
use sea_orm_migration::sea_orm::Iterable;

use entities::account::MailAccountType;
use entities::prelude::Account;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(entities::account::Entity)
                    .if_not_exists()
                    .col(pk_auto(entities::account::Column::Id))
                    .col(string(entities::account::Column::MailAddress))
                    .col(string(entities::account::Column::Status))
                    .col(integer(entities::account::Column::LastUpdateDate))
                    .col(string(entities::account::Column::Token).null())
                    .col(string(entities::account::Column::RefreshToken).null())
                    .col(
                        enumeration_null(entities::account::Column::MailType, Alias::new("mail_type"), MailAccountType::iter())
                    )
                    .col(string(entities::account::Column::NickName).null())
                    .col(big_integer(entities::account::Column::TokenRefreshDate))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }
}
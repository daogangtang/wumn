use cfg_if::cfg_if;
use log::*;
use std::convert::TryFrom;
use crate::entity::EntityManager;
use crate::error::{DbError, ConnectError};
use crate::platform::{Platform, DBPlatform};
use crate::dao_manager::DaoManager;

cfg_if! {if #[cfg(feature = "with-postgres")]{
    use crate::pg::{self, PostgresDB};
}}


pub struct DbManager;

impl DbManager {
    pub fn new() -> Self {
        DbManager
    }

    /// ensure that a connection pool for this db_url exist
    fn db(&mut self, db_url: &str) -> Result<DBPlatform, DbError> {
        info!("ensure db_url: {}", db_url);
        let platform: Result<Platform, _> = TryFrom::try_from(db_url);
        match platform {
            Ok(platform) => match platform {
                #[cfg(feature = "with-postgres")]
                Platform::Postgres => {
                    let conn = pg::init_connection(db_url);
                    Ok(DBPlatform::Postgres(Box::new(PostgresDB(conn))))
                },
                Platform::Unsupported(scheme) => {
                    info!("unsupported");
                    Err(DbError::ConnectError(ConnectError::UnsupportedDb(scheme)))
                }
            },
            Err(e) => Err(DbError::ConnectError(ConnectError::ParseError(e))),
        }
    }

    pub fn em(&mut self, db_url: &str) -> Result<EntityManager, DbError> {
        let db = self.db(db_url)?;
        Ok(EntityManager(db))
    }

    pub fn dm(&mut self, db_url: &str) -> Result<DaoManager, DbError> {
        let db = self.db(db_url)?;
        Ok(DaoManager(db))
    }
}


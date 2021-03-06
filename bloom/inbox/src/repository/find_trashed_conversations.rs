use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_trashed_conversations<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<Vec<entities::Conversation>, Error> {
        const QUERY: &str = "SELECT * FROM inbox_conversations
            WHERE namespace_id = $1 AND trashed_at IS NOT NULL
            ORDER BY last_message_at DESC";

        match sqlx::query_as::<_, entities::Conversation>(QUERY)
            .bind(namespace_id)
            .bind(true)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("inbox.find_trashed_conversations: Finding conversations: {}", &err);
                Err(err.into())
            }
            Ok(conversations) => Ok(conversations),
        }
    }
}

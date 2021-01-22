use crate::repository::Repository;
use kernel::{db::DB, drivers};
use std::sync::Arc;
use stdx::{
    chrono::{DateTime, Utc},
    mail,
    sqlx::{Postgres, Transaction},
    uuid::Uuid,
};

mod create_contact;
mod create_newsletter_list;
mod create_newsletter_message;
mod delete_contact;
mod delete_newsletter_list;
mod delete_newsletter_message;
mod find_archive;
mod find_chatbox_messages;
mod find_chatbox_preferences;
mod find_contact;
mod find_contacts;
mod find_inbox;
mod find_newsletter_list;
mod find_newsletter_lists;
mod find_newsletter_message;
mod find_newsletter_messages;
mod find_spam;
mod find_trash;
mod import_contacts;
mod init_namespace;
mod job_dispatch_send_newsletter_message;
mod job_send_newsletter_message;
mod link_chatbox_contact;
mod send_chatbox_message;
mod send_message;
mod send_newsletter_message;
mod send_test_newsletter_message;
mod subscribe_to_list;
mod unsubscribe_from_list;
mod update_chatbox_preferences;
mod update_contact;
mod update_newsletter_list;
mod update_newsletter_message;
mod validators;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB,
    kernel_service: Arc<kernel::Service>,
    queue: Arc<dyn drivers::Queue>,
    xss: Arc<dyn drivers::XssSanitizer>,
}

impl Service {
    pub fn new(
        kernel_service: Arc<kernel::Service>,
        db: DB,
        queue: Arc<dyn drivers::Queue>,
        xss: Arc<dyn drivers::XssSanitizer>,
    ) -> Service {
        let repo = Repository::new();

        Service {
            db,
            repo,
            kernel_service,
            queue,
            xss,
        }
    }
}

#[async_trait::async_trait]
impl kernel::domain::inbox::Service for Service {
    async fn init_namespace<'c>(
        &self,
        tx: &mut Transaction<'c, Postgres>,
        input: kernel::domain::inbox::InitNamespaceInput,
    ) -> Result<(), kernel::Error> {
        let input = InitNamespaceInput {
            namespace_id: input.namespace_id,
            name: input.name,
        };
        self.init_namespace(tx, input).await
    }
}

#[derive(Debug, Clone)]
pub struct CreateContactInput {
    pub namespace_id: Uuid,
    pub name: String,
    pub birthday: Option<DateTime<Utc>>,
    pub email: String,
    pub pgp_key: String,
    pub phone: String,
    pub address: String,
    pub website: String,
    pub twitter: String,
    pub instagram: String,
    pub facebook: String,
    pub linkedin: String,
    pub skype: String,
    pub telegram: String,
    pub bloom: String,
    pub notes: String,
    pub plan: String,
    pub user_id: String,
}

#[derive(Debug, Clone)]
pub struct CreateNewsletterListInput {
    pub namespace_id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct CreateNewsletterMessageInput {
    pub list_id: Uuid,
    pub name: String,
    pub subject: String,
    pub body: String,
    pub scheduled_for: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct DeleteContactInput {
    pub contact_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct DeleteNewsletterListInput {
    pub list_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct DeleteNewsletterMessageInput {
    pub message_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct ImportContactsInput {
    pub namespace_id: Uuid,
    pub list_id: Option<Uuid>,
    pub contacts_csv: String,
}

#[derive(Debug, Clone)]
pub struct SendNewsletterMessageInput {
    pub message_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct SendTestNewsletterMessageInput {
    pub message_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct UpdateContactInput {
    pub contact_id: Uuid,
    pub name: String,
    pub birthday: Option<DateTime<Utc>>,
    pub email: String,
    pub pgp_key: String,
    pub phone: String,
    pub address: String,
    pub website: String,
    pub twitter: String,
    pub instagram: String,
    pub facebook: String,
    pub linkedin: String,
    pub skype: String,
    pub telegram: String,
    pub bloom: String,
    pub notes: String,
    pub plan: String,
    pub user_id: String,
}

#[derive(Debug, Clone)]
pub struct UpdateNewsletterListInput {
    pub list_id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct UpdateNewsletterMessageInput {
    pub message_id: Uuid,
    pub list_id: Uuid,
    pub name: String,
    pub subject: String,
    pub body: String,
    pub scheduled_for: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct FindContactInput {
    pub contact_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct FindContactsInput {
    pub namespace_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct FindNewsletterListInput {
    pub list_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct FindNewsletterListsInput {
    pub namespace_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct FindNewsletterMessageInput {
    pub message_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct FindNewsletterMessagesInput {
    pub namespace_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct SendMessageInput {
    pub conversation_id: Uuid,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct SendChatboxMessageInput {
    pub namespace_id: Uuid,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct UpdateChatboxPreferencesInput {
    pub namespace_id: Uuid,
    pub color: String,
    pub name: String,
    pub show_branding: bool,
    pub welcome_message: String,
}

#[derive(Debug, Clone)]
pub struct FindChatboxPreferencesInput {
    pub namespace_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct FindChatboxMessagesInput {
    pub namespace_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct LinkChatboxContactInput {
    pub namespace_id: Uuid,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct InitNamespaceInput {
    pub namespace_id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct FindInboxInput {
    pub namespace_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct FindTrashInput {
    pub namespace_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct FindArchiveInput {
    pub namespace_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct FindSpamInput {
    pub namespace_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct SubscribeToListInput {
    pub name: Option<String>,
    pub email: String,
    pub list_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct UnsubscribeFromListInput {
    pub subscription_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct SendNewsletterMessageJobInput {
    pub message_id: Uuid,
    pub to: mail::Address,
    pub from: mail::Address,
    pub contact_id: Option<Uuid>,
}

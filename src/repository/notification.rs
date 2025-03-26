use crate::model::notification::Notification;
use lazy_static::lazy_static;
use std::sync::RwLock;

// Singleton of Database
lazy_static! {
    static ref NOTIFICATIONS: RwLock<Vec<Notification>> = RwLock::new(vec![]);
}

pub struct NotificationRepository;

impl NotificationRepository {}

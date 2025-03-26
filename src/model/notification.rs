use std::fmt::{Display, Formatter, Result};

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Notification {
    pub product_id: String,
    pub product_type: String,
    pub product_title: String,
    pub subscriber_name: String,
    pub subscriber_url: String,
    pub status: String,
}

impl Display for Notification {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.status.to_uppercase().eq("CREATED") {
            write!(
                f,
                "Hello {}, let's try our new {} product: {}, only in BambangShop! Check it out: {}",
                self.subscriber_name,
                self.product_type.to_lowercase(),
                self.product_title,
                self.subscriber_url
            )
        } else if self.status.to_uppercase().eq("DELETED") {
            write!(
                f,
                "Hello {}, we informed that our product called {} already sold out...",
                self.subscriber_name, self.product_title
            )
        } else {
            write!(
                f,
                "Hello {}, let's try our {} product: {}, grab it out before the stock run out! Check it out: {}",
                self.subscriber_name,
                self.product_type.to_lowercase(),
                self.product_title,
                self.subscriber_url
            )
        }
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! QUICK HACK access to the account_activity API.

use serde::Deserialize;

use crate::common::*;
use crate::{auth, links};

//mod fun;
//mod raw;

//pub use self::fun::*;

///A webhook.
#[derive(Debug, Deserialize)]
pub struct Webhook {
    ///ID of this webhook.
    pub id: String,
    ///UTC timestamp from when this webhook was created.
    #[serde(deserialize_with = "deserialize_datetime")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///The URL of the webhook.
    pub url: String,
    ///Whether the webhook validated successfully.
    pub valid: bool,
}

///Information for registering a new webhook
#[derive(Debug, Clone)]
pub struct WebhookSpec {
    ///The URL of the webhook.
    pub url: String,
}

impl WebhookSpec {
    ///Creates a new `WebhookSpec`.
    pub fn new<S: ToString>(url: S) -> Self {
        WebhookSpec {
            url: url.to_string(),
        }
    }

    ///Register the webhook with the specified "environment".
    pub fn register(&self, env_name: &str, token: &auth::Token) -> FutureResponse<Webhook> {
        let url = format!(
            "{}/all/{}/webhooks.json",
            links::activity::ACTIVITY_STEM,
            env_name
        );
        let params = ParamList::new().add_param("url", self.url.clone());
        let req = auth::post(&url, token, Some(&params));
        make_parsed_future(req)
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! QUICK HACK access to the account_activity API.

use serde::Deserialize;

use crate::common::*;
use crate::{
    auth,
    error::{self, Error::InvalidResponse},
    links,
};

//mod fun;
//mod raw;

//pub use self::fun::*;

///A webhook.
#[derive(Debug, Deserialize)]
pub struct Webhook {
    ///ID of this webhook.
    pub id: String,
    ///Timestamp of when this webhook was created -- note that it is
    ///not provided in the standard format. We don't bother parsing it
    ///at the moment. An example value is "2020-03-03 00:47:32 +0000".
    pub created_timestamp: String,
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

///Delete a webhook.
///
///This could/should be a method on `Webhook`, but I don't have enough API
///coverage to make it easy to get `Webhook` instances on which to operate.
pub fn delete_webhook(env_name: &str, webhook_id: &str, token: &auth::Token) -> FutureResponse<()> {
    let url = format!(
        "{}/all/{}/webhooks/{}.json",
        links::activity::ACTIVITY_STEM,
        env_name,
        webhook_id,
    );
    let req = auth::delete(&url, token, None);

    fn parse_resp(full_resp: String, headers: &Headers) -> Result<Response<()>, error::Error> {
        if full_resp.is_empty() {
            rate_headers(headers)
        } else {
            Err(InvalidResponse("Expected empty response", Some(full_resp)))
        }
    }

    make_future(req, parse_resp)
}

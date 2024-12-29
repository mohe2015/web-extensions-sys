use serde::{Deserialize, Serialize};
use tsify_next::{serde_wasm_bindgen, Tsify};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    pub type Cookies;

    #[wasm_bindgen(method, js_name = "get")]
    async fn get_internal(this: &Cookies, details: CookieDetails) -> JsValue;

    #[wasm_bindgen(method, js_name = "set")]
    async fn set_internal(this: &Cookies, details: SetCookieDetails) -> JsValue;
}

impl Cookies {
    pub async fn get(self: &Self, details: CookieDetails) -> Option<Cookie> {
        serde_wasm_bindgen::from_value(self.get_internal(details).await).unwrap()
    }

    pub async fn set(self: &Self, details: SetCookieDetails) -> Option<Cookie> {
        serde_wasm_bindgen::from_value(self.set_internal(details).await).unwrap()
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct SetCookieDetails {
    pub domain: Option<String>,
    pub expiration_date: Option<u64>,
    pub http_only: Option<bool>,
    pub name: Option<String>,
    pub partition_key: Option<CookiePartitionKey>,
    pub path: Option<String>,
    pub same_site: Option<SameSiteStatus>,
    pub secure: Option<bool>,
    pub store_id: Option<String>,
    pub url: String,
    pub value: Option<String>
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct CookieDetails {
    pub name: String,
    pub partition_key: Option<CookiePartitionKey>,
    pub store_id: Option<String>,
    pub url: String,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct CookiePartitionKey {
    pub has_cross_site_ancestor: Option<bool>,
    pub top_level_site: Option<bool>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    pub domain: String,
    pub expiration_date: Option<u64>,
    pub host_only: bool,
    pub http_only: bool,
    pub name: String,
    pub partition_key: Option<CookiePartitionKey>,
    pub path: String,
    pub same_site: SameSiteStatus,
    pub secure: bool,
    pub session: bool,
    pub store_id: String,
    pub value: String
}


#[derive(Tsify, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SameSiteStatus {
    NoRestriction,
    Lax,
    Strict,
    Unspecified,
}
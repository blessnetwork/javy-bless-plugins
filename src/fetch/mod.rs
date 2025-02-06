mod blockless;

use anyhow::{anyhow, bail, Result};
use blockless::BlocklessHttp;
use javy_plugin_api::javy::quickjs::{Object as JSObject, String as JSString, Value};
use javy_plugin_api::javy::Args;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchOptions {
    method: String,
}

impl FetchOptions {
    pub fn new(method: &str) -> Self {
        FetchOptions {
            method: method.into(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub fn bless_fetch_request(args: Args<'_>) -> Result<Value<'_>> {
    let (cx, args) = args.release();
    let (url, options) = extract_http_args(&args, "Javy.Fetch.request")?;

    let url: String = url
        .as_string()
        .ok_or_else(|| anyhow!("url must be a string"))?
        .to_string()
        .map_err(|_| anyhow!("invalid UTF-8 in url"))?;

    let options: String = options
        .as_string()
        .ok_or_else(|| anyhow!("options must be a string"))?
        .to_string()
        .map_err(|_| anyhow!("invalid UTF-8 in options"))?;

    let request_obj: FetchOptions = serde_json::from_str(&options)?;

    // Prepare Response
    // let mut response: HashMap<String, Value<'_>> = HashMap::new();
    let response_obj = JSObject::new(cx.clone()).unwrap();

    let http = BlocklessHttp::open(&url, &request_obj).unwrap();
    let body = String::from_utf8(http.get_all_body().unwrap()).unwrap();
    http.close();

    response_obj.set("ok", Value::new_bool(cx.clone(), true))?;
    response_obj.set(
        "body",
        Value::from_string(JSString::from_str(cx.clone(), &body)?),
    )?;

    Ok(Value::from_object(response_obj))
}

fn extract_http_args<'a, 'js: 'a>(
    args: &'a [Value<'js>],
    for_func: &str,
) -> Result<(&'a Value<'js>, &'a Value<'js>)> {
    let [url, options, ..] = args else {
        bail!(
            r#"{} expects 2 parameters: the URL, the TypedArray buffer,
           the TypedArray byteOffset and the TypedArray byteLength.

           Got: {} parameters."#,
            for_func,
            args.len()
        );
    };

    Ok((url, options))
}

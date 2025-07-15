use anyhow::{anyhow, Result};
use blockless_sdk::http::{HttpClient, HttpResponse, MultipartField};
use javy_plugin_api::javy::{
    hold_and_release,
    quickjs::{prelude::MutFn, Function, Object, String as JSString, Value},
    to_js_error, Args,
};
use std::collections::HashMap;

pub const FETCH_JS: &str = include_str!("fetch.js");

/// A fetch-compliant HTTP client
pub fn bless_fetch_request(args: Args<'_>) -> Result<Value<'_>> {
    let (cx, args) = args.release();

    // Parse URL and options from JavaScript
    if args.is_empty() {
        return Err(anyhow!("URL is required"));
    }

    let url = args[0]
        .as_string()
        .ok_or_else(|| anyhow!("URL must be a string"))?
        .to_string()
        .map_err(|_| anyhow!("invalid UTF-8 in URL"))?;

    // Parse options if provided
    let options = if args.len() > 1 {
        let opts_obj = args[1]
            .as_object()
            .ok_or_else(|| anyhow!("options must be an object"))?;
        parse_fetch_options(opts_obj)?
    } else {
        FetchOptions::default()
    };

    // Create HTTP client
    let client = HttpClient::new();

    // Build request
    let mut request = match options.method.to_uppercase().as_str() {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "PATCH" => client.patch(&url),
        "DELETE" => client.delete(&url),
        "HEAD" => client.head(&url),
        method => client.request(method, &url),
    };

    // Add headers
    for (key, value) in options.headers {
        request = request.header(key, value);
    }

    // Add body
    if let Some(body) = options.body {
        match body {
            BodyInit::Text(text) => request = request.body(text),
            BodyInit::Json(json) => {
                request = request.header("Content-Type", "application/json");
                request = request.body(json);
            }
            BodyInit::FormData(form_data) => {
                let fields: Vec<MultipartField> = form_data
                    .into_iter()
                    .map(|(name, value)| MultipartField::text(name, value))
                    .collect();
                request = request.multipart(fields);
            }
            BodyInit::Binary(data) => request = request.body_bytes(data),
        }
    }

    // Set timeout if specified
    if let Some(timeout) = options.timeout {
        request = request.timeout(timeout);
    }

    // Add signal support (AbortController)
    if let Some(_signal) = options.signal {
        // Note: AbortController support would require additional implementation
        // For now, we'll ignore it but this is where it would be handled
    }

    // Execute request
    let response = request
        .send()
        .map_err(|e| anyhow!("Request failed: {:?}", e))?;

    // Create JavaScript Response object
    create_js_response(cx, response, url)
}

/// Parse fetch options from JavaScript object
fn parse_fetch_options(obj: &Object) -> Result<FetchOptions> {
    let mut options = FetchOptions::default();

    // Method
    if let Ok(Some(method)) = obj.get::<_, Option<String>>("method") {
        options.method = method;
    }

    // Headers
    if let Ok(Some(headers_obj)) = obj.get::<_, Option<Object>>("headers") {
        options.headers = parse_headers(&headers_obj)?;
    }

    // Body
    if let Ok(Some(body_value)) = obj.get::<_, Option<Value>>("body") {
        options.body = Some(parse_body(&body_value)?);
    }

    // Mode
    if let Ok(Some(mode)) = obj.get::<_, Option<String>>("mode") {
        options.mode = mode;
    }

    // Credentials
    if let Ok(Some(credentials)) = obj.get::<_, Option<String>>("credentials") {
        options.credentials = credentials;
    }

    // Cache
    if let Ok(Some(cache)) = obj.get::<_, Option<String>>("cache") {
        options.cache = cache;
    }

    // Redirect
    if let Ok(Some(redirect)) = obj.get::<_, Option<String>>("redirect") {
        options.redirect = redirect;
    }

    // Referrer
    if let Ok(Some(referrer)) = obj.get::<_, Option<String>>("referrer") {
        options.referrer = referrer;
    }

    // Referrer Policy
    if let Ok(Some(referrer_policy)) = obj.get::<_, Option<String>>("referrerPolicy") {
        options.referrer_policy = referrer_policy;
    }

    // Integrity
    if let Ok(Some(integrity)) = obj.get::<_, Option<String>>("integrity") {
        options.integrity = integrity;
    }

    // Keepalive
    if let Ok(Some(keepalive)) = obj.get::<_, Option<bool>>("keepalive") {
        options.keepalive = keepalive;
    }

    // Signal (AbortSignal)
    if let Ok(Some(_signal)) = obj.get::<_, Option<Object>>("signal") {
        // Store signal for potential abort handling
        options.signal = Some(AbortSignal {});
    }

    // Timeout (non-standard but useful)
    if let Ok(Some(timeout)) = obj.get::<_, Option<f64>>("timeout") {
        options.timeout = Some(timeout as u32);
    }

    Ok(options)
}

/// Parse headers from JavaScript object or Headers instance
fn parse_headers(obj: &Object) -> Result<HashMap<String, String>> {
    let mut headers = HashMap::new();

    // Try to iterate over object properties
    // This is a simplified version - in a real implementation we'd need to handle
    // Headers instances and arrays of [key, value] pairs
    let keys_iter = obj.keys::<String>();

    for key_result in keys_iter {
        if let Ok(key) = key_result {
            if let Ok(Some(value)) = obj.get::<_, Option<String>>(&key) {
                headers.insert(key, value);
            }
        }
    }

    Ok(headers)
}

/// Parse body from JavaScript value
fn parse_body(value: &Value) -> Result<BodyInit> {
    if let Some(s) = value.as_string() {
        let body_str = s
            .to_string()
            .map_err(|_| anyhow!("Invalid UTF-8 in body"))?;
        Ok(BodyInit::Text(body_str))
    } else if let Some(obj) = value.as_object() {
        // Check if it's FormData
        if let Ok(Some(true)) = obj.get::<_, Option<bool>>("_isFormData") {
            let form_data = HashMap::new();
            // Parse FormData entries - simplified for now
            // In a real implementation, we'd properly parse the FormData structure
            Ok(BodyInit::FormData(form_data))
        } else if let Ok(Some(constructor_name)) = obj.get::<_, Option<String>>("constructor.name")
        {
            // Check if it's an ArrayBuffer or typed array
            if constructor_name == "ArrayBuffer" || constructor_name.ends_with("Array") {
                return extract_binary_data(obj);
            } else {
                // Assume it's JSON - serialize the object manually
                // In a real implementation, we'd need to properly serialize the JS object
                Ok(BodyInit::Json("{}".to_string()))
            }
        } else {
            // Try to check for ArrayBuffer or Uint8Array by checking for byteLength property
            if obj.get::<_, Option<f64>>("byteLength").is_ok() {
                return extract_binary_data(obj);
            }
            // Assume it's JSON - serialize the object manually
            // In a real implementation, we'd need to properly serialize the JS object
            Ok(BodyInit::Json("{}".to_string()))
        }
    } else if value.is_array() {
        // Handle arrays - check if it's a typed array or regular array
        if let Some(array_obj) = value.as_object() {
            extract_binary_data(array_obj)
        } else {
            Err(anyhow!("Invalid array body"))
        }
    } else {
        Err(anyhow!("Unsupported body type"))
    }
}

/// Extract binary data from ArrayBuffer or typed arrays
fn extract_binary_data(obj: &Object) -> Result<BodyInit> {
    let mut bytes = Vec::new();

    // Check if it's an ArrayBuffer
    if let Ok(Some(byte_length)) = obj.get::<_, Option<f64>>("byteLength") {
        let length = byte_length as usize;

        // For ArrayBuffer, we need to check if there's a way to access the raw bytes
        // First, try to see if it's exposed as an array-like object
        for i in 0..length {
            if let Ok(Some(byte)) = obj.get::<_, Option<f64>>(&i.to_string()) {
                bytes.push(byte as u8);
            } else {
                // If we can't access bytes directly, try alternative methods
                break;
            }
        }
        if bytes.len() == length {
            return Ok(BodyInit::Binary(bytes));
        }
    }

    // Check if it's a typed array (Uint8Array, etc.)
    if let Ok(Some(length)) = obj.get::<_, Option<f64>>("length") {
        let length = length as usize;
        // Try to extract bytes from typed array
        for i in 0..length {
            if let Ok(Some(byte)) = obj.get::<_, Option<f64>>(&i.to_string()) {
                bytes.push(byte as u8);
            } else {
                return Err(anyhow!("Failed to extract byte at index {}", i));
            }
        }
        return Ok(BodyInit::Binary(bytes));
    }

    // If we have a buffer property (for typed arrays), try to access it
    if let Ok(Some(buffer)) = obj.get::<_, Option<Object>>("buffer") {
        // Recurse to handle the underlying ArrayBuffer
        return extract_binary_data(&buffer);
    }
    Err(anyhow!("Unable to extract binary data from object"))
}

/// Create JavaScript Response object from Rust response
fn create_js_response<'js>(
    cx: javy_plugin_api::javy::quickjs::Ctx<'js>,
    response: HttpResponse,
    url: String,
) -> Result<Value<'js>> {
    let response_obj = Object::new(cx.clone())?;

    // Set response properties
    response_obj.set("ok", Value::new_bool(cx.clone(), response.is_success()))?;
    response_obj.set(
        "status",
        Value::new_number(cx.clone(), response.status() as f64),
    )?;
    response_obj.set(
        "statusText",
        Value::from_string(JSString::from_str(
            cx.clone(),
            &get_status_text(response.status()),
        )?),
    )?;
    response_obj.set(
        "url",
        Value::from_string(JSString::from_str(cx.clone(), &url)?),
    )?;
    response_obj.set("redirected", Value::new_bool(cx.clone(), false))?; // TODO: Track redirects
    response_obj.set(
        "type",
        Value::from_string(JSString::from_str(cx.clone(), "basic")?),
    )?;

    // Create headers object
    let headers_obj = Object::new(cx.clone())?;
    for (key, value) in response.headers() {
        headers_obj.set(
            key,
            Value::from_string(JSString::from_str(cx.clone(), value)?),
        )?;
    }
    response_obj.set("headers", Value::from_object(headers_obj))?;

    // Store response data for cloning
    let response_clone = response.clone();
    let body_bytes = response.bytes().to_vec();
    let body_consumed = std::cell::RefCell::new(false);

    // Add text() method
    let body_bytes_clone = body_bytes.clone();
    let body_consumed_clone = body_consumed.clone();
    response_obj.set(
        "text",
        Function::new(
            cx.clone(),
            MutFn::new(move |cx, args| {
                let (cx, _args) = hold_and_release!(cx, args);

                if *body_consumed_clone.borrow() {
                    return Err(to_js_error(cx, anyhow!("Body already consumed")));
                }
                *body_consumed_clone.borrow_mut() = true;

                String::from_utf8(body_bytes_clone.clone())
                    .map_err(|_| anyhow!("Invalid UTF-8 in response body"))
                    .and_then(|text| {
                        JSString::from_str(cx.clone(), &text)
                            .map(Value::from_string)
                            .map_err(|e| anyhow!("Failed to create JS string: {:?}", e))
                    })
                    .map_err(|e| to_js_error(cx, e))
            }),
        ),
    )?;

    // Add json() method
    let body_bytes_clone = body_bytes.clone();
    let body_consumed_clone = body_consumed.clone();
    response_obj.set(
        "json",
        Function::new(
            cx.clone(),
            MutFn::new(move |cx, args| {
                let (cx, _args) = hold_and_release!(cx, args);

                if *body_consumed_clone.borrow() {
                    return Err(to_js_error(cx, anyhow!("Body already consumed")));
                }
                *body_consumed_clone.borrow_mut() = true;

                String::from_utf8(body_bytes_clone.clone())
                    .map_err(|_| anyhow!("Invalid UTF-8 in response body"))
                    .and_then(|text| {
                        let js_code = format!("({})", text);
                        cx.eval::<Value, _>(js_code.as_bytes())
                            .map_err(|e| anyhow!("Failed to parse JSON: {:?}", e))
                    })
                    .map_err(|e| to_js_error(cx, e))
            }),
        ),
    )?;

    // Add arrayBuffer() method
    let body_bytes_clone = body_bytes.clone();
    let body_consumed_clone = body_consumed.clone();
    response_obj.set(
        "arrayBuffer",
        Function::new(
            cx.clone(),
            MutFn::new(move |cx, args| {
                let (cx, _args) = hold_and_release!(cx, args);

                if *body_consumed_clone.borrow() {
                    return Err(to_js_error(cx, anyhow!("Body already consumed")));
                }
                *body_consumed_clone.borrow_mut() = true;

                // Create ArrayBuffer from bytes
                // This is a simplified version - proper implementation would create actual ArrayBuffer
                let array_obj = Object::new(cx.clone())?;
                array_obj.set(
                    "byteLength",
                    Value::new_number(cx.clone(), body_bytes_clone.len() as f64),
                )?;

                // Store bytes as array
                for (i, byte) in body_bytes_clone.iter().enumerate() {
                    array_obj.set(&i.to_string(), Value::new_number(cx.clone(), *byte as f64))?;
                }

                Ok(Value::from_object(array_obj))
            }),
        ),
    )?;

    // Add blob() method (simplified - returns object with size and type)
    let body_bytes_clone = body_bytes.clone();
    let body_consumed_clone = body_consumed.clone();
    response_obj.set(
        "blob",
        Function::new(
            cx.clone(),
            MutFn::new(move |cx, args| {
                let (cx, _args) = hold_and_release!(cx, args);

                if *body_consumed_clone.borrow() {
                    return Err(to_js_error(cx, anyhow!("Body already consumed")));
                }
                *body_consumed_clone.borrow_mut() = true;

                let blob_obj = Object::new(cx.clone())?;
                blob_obj.set(
                    "size",
                    Value::new_number(cx.clone(), body_bytes_clone.len() as f64),
                )?;
                blob_obj.set(
                    "type",
                    Value::from_string(JSString::from_str(cx.clone(), "application/octet-stream")?),
                )?;

                Ok(Value::from_object(blob_obj))
            }),
        ),
    )?;

    // Add clone() method
    let response_for_clone = response_clone.clone();
    let url_for_clone = url.clone();
    response_obj.set(
        "clone",
        Function::new(
            cx.clone(),
            MutFn::new(move |cx, args| {
                let (cx, _args) = hold_and_release!(cx, args);
                match create_js_response(
                    cx.clone(),
                    response_for_clone.clone(),
                    url_for_clone.clone(),
                ) {
                    Ok(cloned_response) => Ok(cloned_response),
                    Err(e) => Err(to_js_error(cx, e)),
                }
            }),
        ),
    )?;

    Ok(Value::from_object(response_obj))
}

/// Get status text for HTTP status code
fn get_status_text(status: u16) -> &'static str {
    match status {
        100 => "Continue",
        101 => "Switching Protocols",
        102 => "Processing",
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        203 => "Non-Authoritative Information",
        204 => "No Content",
        205 => "Reset Content",
        206 => "Partial Content",
        207 => "Multi-Status",
        208 => "Already Reported",
        226 => "IM Used",
        300 => "Multiple Choices",
        301 => "Moved Permanently",
        302 => "Found",
        303 => "See Other",
        304 => "Not Modified",
        305 => "Use Proxy",
        307 => "Temporary Redirect",
        308 => "Permanent Redirect",
        400 => "Bad Request",
        401 => "Unauthorized",
        402 => "Payment Required",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        406 => "Not Acceptable",
        407 => "Proxy Authentication Required",
        408 => "Request Timeout",
        409 => "Conflict",
        410 => "Gone",
        411 => "Length Required",
        412 => "Precondition Failed",
        413 => "Payload Too Large",
        414 => "URI Too Long",
        415 => "Unsupported Media Type",
        416 => "Range Not Satisfiable",
        417 => "Expectation Failed",
        418 => "I'm a teapot",
        421 => "Misdirected Request",
        422 => "Unprocessable Entity",
        423 => "Locked",
        424 => "Failed Dependency",
        425 => "Too Early",
        426 => "Upgrade Required",
        428 => "Precondition Required",
        429 => "Too Many Requests",
        431 => "Request Header Fields Too Large",
        451 => "Unavailable For Legal Reasons",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        505 => "HTTP Version Not Supported",
        506 => "Variant Also Negotiates",
        507 => "Insufficient Storage",
        508 => "Loop Detected",
        510 => "Not Extended",
        511 => "Network Authentication Required",
        _ => "Unknown",
    }
}

/// Fetch options structure
#[derive(Debug, Clone)]
struct FetchOptions {
    method: String,
    headers: HashMap<String, String>,
    body: Option<BodyInit>,
    mode: String,
    credentials: String,
    cache: String,
    redirect: String,
    referrer: String,
    referrer_policy: String,
    integrity: String,
    keepalive: bool,
    signal: Option<AbortSignal>,
    timeout: Option<u32>,
}

impl Default for FetchOptions {
    fn default() -> Self {
        Self {
            method: "GET".to_string(),
            headers: HashMap::new(),
            body: None,
            mode: "cors".to_string(),
            credentials: "same-origin".to_string(),
            cache: "default".to_string(),
            redirect: "follow".to_string(),
            referrer: "about:client".to_string(),
            referrer_policy: "".to_string(),
            integrity: "".to_string(),
            keepalive: false,
            signal: None,
            timeout: None,
        }
    }
}

/// Body initialization types
#[derive(Debug, Clone)]
enum BodyInit {
    Text(String),
    Json(String),
    FormData(HashMap<String, String>),
    Binary(Vec<u8>),
}

/// Placeholder for AbortSignal support
#[derive(Debug, Clone)]
struct AbortSignal {}

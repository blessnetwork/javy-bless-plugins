use anyhow::{anyhow, Result};
use blockless_sdk::{BlessCrawl, CrawlOptions, MapOptions, ScrapeOptions, Viewport};
use javy_plugin_api::javy::{
    hold, hold_and_release,
    quickjs::{prelude::MutFn, Function, Object, Value},
    to_js_error, Args,
};
use std::collections::HashMap;
use std::sync::Arc;

/// Create a BlessCrawl instance from JavaScript
pub fn bless_crawl(args: Args<'_>) -> Result<Value<'_>> {
    let (cx, args) = args.release();

    // optional config can be provided in constructor
    // Wrap in Arc for sharing across closures
    let scrape_config = if args.len() > 0 {
        let config_obj = args[0]
            .as_object()
            .ok_or_else(|| anyhow!("config must be an object"))?;
        Arc::new(Some(parse_options(config_obj)))
    } else {
        Arc::new(None)
    };

    // Create JavaScript object wrapper
    let instance = Object::new(cx.clone())?;

    // Clone Arc for the scrape closure
    let scrape_config_clone = scrape_config.clone();
    instance.set(
        "scrape",
        Function::new(
            cx.clone(),
            MutFn::new(move |cx, args| {
                let (cx, args) = hold_and_release!(cx, args);

                let scrape_fn = |args: Args<'_>| {
                    let (_cx, args) = args.release();

                    if args.is_empty() {
                        return Err(anyhow!("URL is required"));
                    }

                    let url = args[0]
                        .as_string()
                        .ok_or_else(|| anyhow!("URL must be a string"))?
                        .to_string()
                        .map_err(|_| anyhow!("invalid UTF-8 in URL"))?;

                    // Check if args[1] is provided, otherwise use scrape_config, otherwise error
                    let scrape_options = if args.len() > 1 {
                        let opts_obj = args[1]
                            .as_object()
                            .ok_or_else(|| anyhow!("options must be an object"))?;
                        parse_options(opts_obj)
                    } else if let Some(config) = scrape_config_clone.as_ref() {
                        config.clone()
                    } else {
                        return Err(anyhow!(
                            "No scrape options provided and no default config available"
                        ));
                    };

                    let result = BlessCrawl::with_config(scrape_options)
                        .unwrap()
                        .scrape(&url, None)
                        .map_err(|e| anyhow!("Scrape failed: {:?}", e))?;

                    // Serialize the result to JSON and parse it as a JavaScript value
                    let json_str = serde_json::to_string(&result)
                        .map_err(|e| anyhow!("Failed to serialize result: {:?}", e))?;
                    let js_code = format!("({})", json_str);
                    let js_value = cx.eval::<Value, _>(js_code.as_bytes()).map_err(|e| {
                        anyhow!("Failed to parse JSON as JavaScript value: {:?}", e)
                    })?;

                    Ok(js_value)
                };

                scrape_fn(hold!(cx.clone(), args)).map_err(|e| to_js_error(cx, e))
            }),
        ),
    )?;

    // Add map method
    let scrape_config_clone = scrape_config.clone();
    instance.set(
        "map",
        Function::new(
            cx.clone(),
            MutFn::new(move |cx, args| {
                let (cx, args) = hold_and_release!(cx, args);

                let map_fn = |args: Args<'_>| {
                    let (_cx, args) = args.release();

                    if args.is_empty() {
                        return Err(anyhow!("URL is required"));
                    }

                    let url = args[0]
                        .as_string()
                        .ok_or_else(|| anyhow!("URL must be a string"))?
                        .to_string()
                        .map_err(|_| anyhow!("invalid UTF-8 in URL"))?;

                    let (scrape_options, map_options) = if args.len() > 1 {
                        let opts_obj = args[1]
                            .as_object()
                            .ok_or_else(|| anyhow!("options must be an object"))?;
                        (parse_options(&opts_obj), parse_map_options(&opts_obj))
                    } else if let Some(config) = scrape_config_clone.as_ref() {
                        (config.clone(), MapOptions::default())
                    } else {
                        return Err(anyhow!(
                            "No scrape options provided and no default config available"
                        ));
                    };

                    let result = BlessCrawl::with_config(scrape_options)
                        .unwrap()
                        .map(&url, Some(map_options))
                        .map_err(|e| anyhow!("Map failed: {:?}", e))?;

                    let json_str = serde_json::to_string(&result)
                        .map_err(|e| anyhow!("Failed to serialize result: {:?}", e))?;

                    let js_code = format!("({})", json_str);
                    let js_value = cx.eval::<Value, _>(js_code.as_bytes()).map_err(|e| {
                        anyhow!("Failed to parse JSON as JavaScript value: {:?}", e)
                    })?;

                    Ok(js_value)
                };

                map_fn(hold!(cx.clone(), args)).map_err(|e| to_js_error(cx, e))
            }),
        ),
    )?;

    // Add crawl method
    let scrape_config_clone = scrape_config.clone();
    instance.set(
        "crawl",
        Function::new(
            cx.clone(),
            MutFn::new(move |cx, args| {
                let (cx, args) = hold_and_release!(cx, args);

                let crawl_fn = |args: Args<'_>| {
                    let (_cx, args) = args.release();

                    if args.is_empty() {
                        return Err(anyhow!("URL is required"));
                    }

                    let url = args[0]
                        .as_string()
                        .ok_or_else(|| anyhow!("URL must be a string"))?
                        .to_string()
                        .map_err(|_| anyhow!("invalid UTF-8 in URL"))?;

                    let (scrape_options, crawl_options) = if args.len() > 1 {
                        let opts_obj = args[1]
                            .as_object()
                            .ok_or_else(|| anyhow!("options must be an object"))?;
                        (parse_options(&opts_obj), parse_crawl_options(&opts_obj))
                    } else if let Some(config) = scrape_config_clone.as_ref() {
                        (config.clone(), CrawlOptions::default())
                    } else {
                        return Err(anyhow!(
                            "No scrape options provided and no default config available"
                        ));
                    };

                    let result = BlessCrawl::with_config(scrape_options)
                        .unwrap()
                        .crawl(&url, Some(crawl_options))
                        .map_err(|e| anyhow!("Crawl failed: {:?}", e))?;

                    let json_str = serde_json::to_string(&result)
                        .map_err(|e| anyhow!("Failed to serialize result: {:?}", e))?;

                    let js_code = format!("({})", json_str);
                    let js_value = cx.eval::<Value, _>(js_code.as_bytes()).map_err(|e| {
                        anyhow!("Failed to parse JSON as JavaScript value: {:?}", e)
                    })?;

                    Ok(js_value)
                };

                crawl_fn(hold!(cx.clone(), args)).map_err(|e| to_js_error(cx, e))
            }),
        ),
    )?;

    Ok(Value::from_object(instance))
}

/// Parse JavaScript object into ScrapeOptions
fn parse_options(obj: &Object) -> ScrapeOptions {
    // Parse all scrape options from the object
    let mut scrape_options = ScrapeOptions::default();
    if let Ok(Some(timeout)) = obj.get::<_, Option<f64>>("timeout") {
        scrape_options.timeout = timeout as u32;
    }
    if let Ok(Some(wait_time)) = obj.get::<_, Option<f64>>("wait_time") {
        scrape_options.wait_time = wait_time as u32;
    }
    if let Ok(Some(include_tags)) = obj.get::<_, Option<Vec<String>>>("include_tags") {
        scrape_options.include_tags = Some(include_tags);
    }
    if let Ok(Some(exclude_tags)) = obj.get::<_, Option<Vec<String>>>("exclude_tags") {
        scrape_options.exclude_tags = Some(exclude_tags);
    }
    if let Ok(Some(only_main_content)) = obj.get::<_, Option<bool>>("only_main_content") {
        scrape_options.only_main_content = only_main_content;
    }
    if let Ok(Some(format)) = obj.get::<_, Option<String>>("format") {
        scrape_options.format = format.parse().unwrap_or_default();
    }
    if let Ok(Some(viewport_obj)) = obj.get::<_, Option<Object>>("viewport") {
        let mut viewport = Viewport::default();
        if let Ok(Some(width)) = viewport_obj.get::<_, Option<u32>>("width") {
            viewport.width = Some(width);
        }
        if let Ok(Some(height)) = viewport_obj.get::<_, Option<u32>>("height") {
            viewport.height = Some(height);
        }
        scrape_options.viewport = Some(viewport);
    }
    if let Ok(Some(user_agent)) = obj.get::<_, Option<String>>("user_agent") {
        scrape_options.user_agent = Some(user_agent);
    }
    if let Ok(Some(headers_obj)) = obj.get::<_, Option<HashMap<String, String>>>("headers") {
        scrape_options.headers = Some(headers_obj);
    }
    scrape_options
}

/// Parse JavaScript object into MapOptions
fn parse_map_options(obj: &Object) -> MapOptions {
    let mut options = MapOptions::default();
    if let Ok(Some(base_url)) = obj.get::<_, Option<String>>("base_url") {
        options.base_url = Some(base_url);
    }
    if let Ok(Some(link_types)) = obj.get::<_, Option<Vec<String>>>("link_types") {
        options.link_types = Some(link_types);
    }
    if let Ok(Some(filter_extensions)) = obj.get::<_, Option<Vec<String>>>("filter_extensions") {
        options.filter_extensions = Some(filter_extensions);
    }
    options
}

/// Parse JavaScript object into CrawlOptions
fn parse_crawl_options(obj: &Object) -> CrawlOptions {
    let mut options = CrawlOptions::default();
    if let Ok(Some(limit)) = obj.get::<_, Option<f64>>("limit") {
        options.limit = Some(limit as u32);
    }
    if let Ok(Some(max_depth)) = obj.get::<_, Option<u8>>("max_depth") {
        options.max_depth = Some(max_depth);
    }
    if let Ok(Some(exclude_paths)) = obj.get::<_, Option<Vec<String>>>("exclude_paths") {
        options.exclude_paths = Some(exclude_paths);
    }
    if let Ok(Some(include_paths)) = obj.get::<_, Option<Vec<String>>>("include_paths") {
        options.include_paths = Some(include_paths);
    }
    if let Ok(Some(follow_external)) = obj.get::<_, Option<bool>>("follow_external") {
        options.follow_external = Some(follow_external);
    }
    if let Ok(Some(delay)) = obj.get::<_, Option<u32>>("delay_between_requests") {
        options.delay_between_requests = Some(delay);
    }
    if let Ok(Some(parallel)) = obj.get::<_, Option<u32>>("parallel_requests") {
        options.parallel_requests = Some(parallel);
    }
    options
}

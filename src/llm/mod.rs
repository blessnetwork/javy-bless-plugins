use anyhow::{anyhow, Result};
use blockless_sdk::llm::{BlocklessLlm, LlmOptions, Models};
use javy_plugin_api::javy::{
    hold, hold_and_release,
    quickjs::{prelude::MutFn, Function, Object, String as JSString, Value},
    to_js_error, Args,
};
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

pub fn supported_models_object<'js>(
    cx: &javy_plugin_api::javy::quickjs::Ctx<'js>,
) -> Result<Object<'js>> {
    let models = Object::new(cx.clone())?;

    // Llama 3.2 1B
    let llama_3_2_1b = Object::new(cx.clone())?;
    llama_3_2_1b.set("DEFAULT", "Llama-3.2-1B-Instruct")?;
    llama_3_2_1b.set("Q6_K", "Llama-3.2-1B-Instruct-Q6_K")?;
    llama_3_2_1b.set("Q4F16_1", "Llama-3.2-1B-Instruct-q4f16_1")?;
    models.set("LLAMA_3_2_1B", Value::from_object(llama_3_2_1b))?;

    // Llama 3.2 3B
    let llama_3_2_3b = Object::new(cx.clone())?;
    llama_3_2_3b.set("DEFAULT", "Llama-3.2-3B-Instruct")?;
    llama_3_2_3b.set("Q6_K", "Llama-3.2-3B-Instruct-Q6_K")?;
    llama_3_2_3b.set("Q4F16_1", "Llama-3.2-3B-Instruct-q4f16_1")?;
    models.set("LLAMA_3_2_3B", Value::from_object(llama_3_2_3b))?;

    // Mistral 7B
    let mistral_7b = Object::new(cx.clone())?;
    mistral_7b.set("DEFAULT", "Mistral-7B-Instruct-v0.3")?;
    mistral_7b.set("Q4F16_1", "Mistral-7B-Instruct-v0.3-q4f16_1")?;
    models.set("MISTRAL_7B", Value::from_object(mistral_7b))?;

    // Mixtral 8x7B
    let mixtral_8x7b = Object::new(cx.clone())?;
    mixtral_8x7b.set("DEFAULT", "Mixtral-8x7B-Instruct-v0.1")?;
    mixtral_8x7b.set("Q4F16_1", "Mixtral-8x7B-Instruct-v0.1-q4f16_1")?;
    models.set("MIXTRAL_8X7B", Value::from_object(mixtral_8x7b))?;

    // Gemma models
    let gemma_2_2b = Object::new(cx.clone())?;
    gemma_2_2b.set("DEFAULT", "gemma-2-2b-it")?;
    gemma_2_2b.set("Q4F16_1", "gemma-2-2b-it-q4f16_1")?;
    models.set("GEMMA_2_2B", Value::from_object(gemma_2_2b))?;

    let gemma_2_7b = Object::new(cx.clone())?;
    gemma_2_7b.set("DEFAULT", "gemma-2-27b-it")?;
    gemma_2_7b.set("Q4F16_1", "gemma-2-27b-it-q4f16_1")?;
    models.set("GEMMA_2_7B", Value::from_object(gemma_2_7b))?;

    let gemma_2_9b = Object::new(cx.clone())?;
    gemma_2_9b.set("DEFAULT", "gemma-2-9b-it")?;
    gemma_2_9b.set("Q4F16_1", "gemma-2-9b-it-q4f16_1")?;
    models.set("GEMMA_2_9B", Value::from_object(gemma_2_9b))?;

    Ok(models)
}

pub fn bless_llm_plugin(args: Args<'_>) -> Result<Value<'_>> {
    let (cx, args) = args.release();
    if args.len() != 1 {
        return Err(anyhow!("model name required"));
    }

    let model_name = args[0]
        .as_string()
        .ok_or_else(|| anyhow!("model name must be a string"))?
        .to_string()
        .map_err(|_| anyhow!("invalid UTF-8 in model name"))?;

    let model =
        Models::from_str(&model_name).map_err(|_| anyhow!("Invalid model name: {}", model_name))?;

    // Create BlocklessLlm instance using SDK
    let llm = Arc::new(Mutex::new(BlocklessLlm::new(model).unwrap()));

    // Convert to QuickJS object and expose SDK methods
    let instance = Object::new(cx.clone())?;

    // Expose the models object on the instance
    instance.set("MODELS", Value::from_object(supported_models_object(&cx)?))?;

    let llm_ref = Arc::clone(&llm);
    instance.set(
        "setOptions",
        Function::new(
            cx.clone(),
            MutFn::new(move |cx, args| {
                let (cx, args) = hold_and_release!(cx, args);

                let set_options = |args: Args<'_>| {
                    let (_cx, args) = args.release();

                    if args.len() != 1 {
                        return Err(anyhow!("options must be an object"));
                    }

                    let opts_obj = args[0]
                        .as_object()
                        .ok_or_else(|| anyhow!("options must be an object"))?;

                    let system_message = opts_obj.get::<_, Option<String>>("system_message")?;
                    let tools_sse_urls =
                        opts_obj.get::<_, Option<Vec<String>>>("tools_sse_urls")?;
                    let temperature = opts_obj.get::<_, Option<f64>>("temperature")?;
                    let top_p = opts_obj.get::<_, Option<f64>>("top_p")?;
                    let options = LlmOptions {
                        system_message,
                        tools_sse_urls,
                        temperature: temperature.map(|t| t as f32),
                        top_p: top_p.map(|t| t as f32),
                    };

                    llm_ref
                        .lock()
                        .unwrap()
                        .set_options(options)
                        .map(|_| Value::new_undefined(cx.clone()))
                        .map_err(|e| anyhow!("Set options failed: {:?}", e))
                };

                set_options(hold!(cx.clone(), args)).map_err(|e| to_js_error(cx, e))
            }),
        ),
    )?;

    let llm_ref = Arc::clone(&llm);
    instance.set(
        "getOptions",
        Function::new(
            cx.clone(),
            MutFn::new(move |cx, args| {
                let (cx, args) = hold_and_release!(cx, args);

                let get_options = |args: Args<'_>| {
                    let (_cx, _args) = args.release();

                    let options = llm_ref.lock().unwrap().get_options().unwrap();

                    let opts_obj = Object::new(cx.clone())?;

                    // Handle Option<String> for system_message
                    if let Some(system_msg) = &options.system_message {
                        opts_obj.set(
                            "system_message",
                            Value::from_string(JSString::from_str(cx.clone(), system_msg)?),
                        )?;
                    }

                    // Handle Option<Vec<String>> for tools_sse_urls
                    if let Some(urls) = &options.tools_sse_urls {
                        let urls_array = Object::new(cx.clone())?;

                        // Set length property for array-like behavior
                        urls_array
                            .set("length", Value::new_number(cx.clone(), urls.len() as f64))?;

                        // Add each URL as a numerically-indexed property
                        for (i, url) in urls.iter().enumerate() {
                            urls_array.set(
                                &i.to_string(),
                                Value::from_string(JSString::from_str(cx.clone(), url)?),
                            )?;
                        }
                        opts_obj.set("tools_sse_urls", Value::from_object(urls_array))?;
                    }

                    if let Some(temperature) = options.temperature {
                        opts_obj.set(
                            "temperature",
                            Value::new_number(cx.clone(), temperature.into()),
                        )?;
                    }
                    if let Some(top_p) = options.top_p {
                        opts_obj.set("top_p", Value::new_number(cx.clone(), top_p.into()))?;
                    }

                    Ok(Value::from_object(opts_obj))
                };

                get_options(hold!(cx.clone(), args)).map_err(|e| to_js_error(cx, e))
            }),
        ),
    )?;

    let llm_ref = Arc::clone(&llm);
    instance.set(
        "chat",
        Function::new(
            cx.clone(),
            MutFn::new(move |cx, args| {
                let (cx, args) = hold_and_release!(cx, args);

                let chat = |args: Args<'_>| {
                    let (_cx, args) = args.release();

                    if args.len() != 1 {
                        return Err(anyhow!("prompt required"));
                    }

                    let prompt = args[0]
                        .as_string()
                        .ok_or_else(|| anyhow!("prompt required"))?
                        .to_string()
                        .map_err(|_| anyhow!("invalid UTF-8 in prompt"))?;
                    llm_ref
                        .lock()
                        .unwrap()
                        .chat_request(&prompt)
                        .map(|res| {
                            let js_string = JSString::from_str(cx.clone(), &res).unwrap();
                            Value::from_string(js_string)
                        })
                        .map_err(|e| anyhow!("Chat request failed: {:?}", e))
                };

                chat(hold!(cx.clone(), args)).map_err(|e| to_js_error(cx, e))
            }),
        ),
    )?;

    Ok(Value::from_object(instance))
}

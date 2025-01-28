//! Javy Bless Plugins

use anyhow::{anyhow, Result};
use blockless_sdk::{BlocklessLlm, LlmOptions};
use javy_plugin_api::{
    import_namespace,
    javy::{
        hold, hold_and_release,
        quickjs::{prelude::MutFn, Function, Object, String as JSString, Value},
        to_js_error, Args,
    },
    Config,
};
use std::sync::{Arc, Mutex};

import_namespace!("bless_plugins");

#[export_name = "initialize_runtime"]
pub extern "C" fn initialize_runtime() {
    let config = Config::default();
    javy_plugin_api::initialize_runtime(config, |runtime| {
        runtime
            .context()
            .with(|ctx| {
                ctx.globals().set(
                    "BlessLLM",
                    Function::new(
                        ctx.clone(),
                        MutFn::new(move |cx, args| {
                            let (cx, args) = hold_and_release!(cx, args);
                            bless_llm_plugin(hold!(cx.clone(), args))
                                .map_err(|e| to_js_error(cx, e))
                        }),
                    )?,
                )?;
                Ok::<_, anyhow::Error>(())
            })
            .unwrap();

        runtime
    })
    .unwrap();
}

fn bless_llm_plugin(args: Args<'_>) -> Result<Value<'_>> {
    let (cx, args) = args.release();
    if args.len() != 1 {
        return Err(anyhow!("model name required"));
    }

    let model_name = args[0]
        .as_string()
        .ok_or_else(|| anyhow!("model name must be a string"))?
        .to_string()
        .map_err(|_| anyhow!("invalid UTF-8 in model name"))?;

    // Create BlocklessLlm instance using SDK
    let llm = Arc::new(Mutex::new(BlocklessLlm::new(&model_name).unwrap()));

    // Convert to QuickJS object and expose SDK methods
    let instance = Object::new(cx.clone())?;

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

                    let system_message = opts_obj.get::<_, String>("system_message")?;
                    let options = LlmOptions {
                        system_message,
                        ..Default::default()
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

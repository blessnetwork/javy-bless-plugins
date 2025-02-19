use javy_plugin_api::{
    import_namespace,
    javy::{
        hold, hold_and_release,
        quickjs::{prelude::MutFn, Function},
        to_js_error, Args,
    },
    Config,
};

#[cfg(feature = "crypto")]
pub mod crypto;
#[cfg(feature = "fetch")]
pub mod fetch;
#[cfg(feature = "llm")]
pub mod llm;

#[cfg(feature = "crypto")]
use crypto::bless_get_random_values;
#[cfg(feature = "fetch")]
use fetch::bless_fetch_request;
#[cfg(feature = "llm")]
use llm::bless_llm_plugin;

import_namespace!("bless_core_plugins");

#[export_name = "initialize_runtime"]
pub extern "C" fn initialize_runtime() {
    let mut config = Config::default();
    config.event_loop(true);
    config.javy_stream_io(true);
    config.text_encoding(true);

    javy_plugin_api::initialize_runtime(config, |runtime| {
        runtime
            .context()
            .with(|ctx| {
                #[cfg(feature = "crypto")]
                ctx.globals().set(
                    "__javy_crypto_get_random_values",
                    Function::new(
                        ctx.clone(),
                        MutFn::new(move |cx, args| {
                            let (cx, args) = hold_and_release!(cx, args);
                            bless_get_random_values(hold!(cx.clone(), args))
                                .map_err(|e| to_js_error(cx, e))
                        }),
                    )?,
                )?;

                #[cfg(feature = "fetch")]
                ctx.globals().set(
                    "__javy_fetchio_request",
                    Function::new(
                        ctx.clone(),
                        MutFn::new(move |cx, args| {
                            let (cx, args) = hold_and_release!(cx, args);
                            bless_fetch_request(hold!(cx.clone(), args))
                                .map_err(|e| to_js_error(cx, e))
                        }),
                    )?,
                )?;

                #[cfg(feature = "llm")]
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

                // Expose the suppported models object globally for JS
                #[cfg(feature = "llm")]
                ctx.globals().set(
                    "MODELS",
                    javy_plugin_api::javy::quickjs::Value::from_object(
                        llm::supported_models_object(&ctx)?,
                    ),
                )?;

                #[cfg(feature = "crypto")]
                ctx.eval::<(), _>(include_str!("crypto/crypto.js"))?;
                #[cfg(feature = "fetch")]
                ctx.eval::<(), _>(include_str!("fetch/fetch.js"))?;
                Ok::<_, anyhow::Error>(())
            })
            .unwrap();

        runtime
    })
    .unwrap();
}

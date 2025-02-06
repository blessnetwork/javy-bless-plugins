//! Javy Bless Plugins
pub mod crypto;
pub mod fetch;
pub mod llm;

use crypto::bless_get_random_values;

use fetch::bless_fetch_request;
use javy_plugin_api::{
    import_namespace,
    javy::{
        hold, hold_and_release,
        quickjs::{prelude::MutFn, Function},
        to_js_error, Args,
    },
    Config,
};

// use llm::bless_llm_plugin;

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

                // ctx.globals().set(
                //     "BlessLLM",
                //     Function::new(
                //         ctx.clone(),
                //         MutFn::new(move |cx, args| {
                //             let (cx, args) = hold_and_release!(cx, args);
                //             bless_llm_plugin(hold!(cx.clone(), args))
                //                 .map_err(|e| to_js_error(cx, e))
                //         }),
                //     )?,
                // )?;

                ctx.eval::<(), _>(include_str!("crypto/crypto.js"))?;
                ctx.eval::<(), _>(include_str!("fetch/fetch.js"))?;

                Ok::<_, anyhow::Error>(())
            })
            .unwrap();

        runtime
    })
    .unwrap();
}

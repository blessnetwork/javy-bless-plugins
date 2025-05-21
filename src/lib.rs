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
#[cfg(feature = "wasi")]
pub mod wasi;
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

                #[cfg(feature = "wasi")]
                {
                    macro_rules! bind {
                        (function, $l: ident) => {
                            let name = concat!("__javy_", stringify!($l));
                            ctx.globals().set(
                                name,
                                Function::new(
                                    ctx.clone(),
                                    MutFn::new(move |cx, args| {
                                        let (cx, args) = hold_and_release!(cx, args);
                                        wasi::$l(hold!(cx.clone(), args))
                                            .map_err(|e| to_js_error(cx, e))
                                    }),
                                )?,
                            )?;
                        };
                    }
                    bind!(function, wasi_preview1_open);
                    bind!(function, wasi_preview1_fd_prestat_dir_name);
                    bind!(function, wasi_preview1_path_create_directory);
                    bind!(function, wasi_preview1_path_remove_directory);
                    bind!(function, wasi_preview1_path_unlink_file);
                    bind!(function, wasi_preview1_close);
                    bind!(function, wasi_preview1_path_symlink);
                    bind!(function, wasi_preview1_path_link);
                }

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
                #[cfg(feature = "wasi")]
                ctx.eval::<(), _>(include_str!("wasi/preview_1.js"))?;
                Ok::<_, anyhow::Error>(())
            })
            .unwrap();

        runtime
    })
    .unwrap();
}

#[allow(unused_imports)]
use javy_plugin_api::{
    import_namespace,
    javy::{
        hold, hold_and_release,
        quickjs::{prelude::MutFn, Ctx, Function, Result},
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
#[cfg(feature = "wasip1")]
pub mod wasi;

#[cfg(feature = "runtime")]
import_namespace!("bless_core_plugins");

#[cfg(feature = "runtime")]
#[export_name = "initialize_runtime"]
pub extern "C" fn initialize_runtime() {
    let mut config = Config::default();
    config.event_loop(true);
    config.javy_stream_io(true);
    config.text_encoding(true);

    #[allow(unused_variables)]
    javy_plugin_api::initialize_runtime(config, |runtime| {
        runtime
            .context()
            .with(|ctx| {
                #[cfg(feature = "crypto")]
                set_crypto_globals(&ctx)?;
                #[cfg(feature = "fetch")]
                set_fetch_globals(&ctx)?;
                #[cfg(feature = "wasip1")]
                set_wasi_globals(&ctx)?;
                #[cfg(feature = "llm")]
                set_llm_globals(&ctx)?;
                Ok::<_, anyhow::Error>(())
            })
            .unwrap();

        runtime
    })
    .unwrap();
}

#[cfg(feature = "crypto")]
pub fn set_crypto_globals(ctx: &Ctx<'_>) -> Result<()> {
    ctx.globals().set(
        "__javy_crypto_get_random_values",
        Function::new(
            ctx.clone(),
            MutFn::new(move |cx, args| {
                let (cx, args) = hold_and_release!(cx, args);
                crypto::bless_get_random_values(hold!(cx.clone(), args))
                    .map_err(|e| to_js_error(cx, e))
            }),
        )?,
    )?;
    ctx.eval::<(), _>(include_str!("crypto/crypto.js"))?;
    Ok(())
}

#[cfg(feature = "fetch")]
pub fn set_fetch_globals(ctx: &Ctx<'_>) -> Result<()> {
    ctx.globals().set(
        "__javy_fetchio_request",
        Function::new(
            ctx.clone(),
            MutFn::new(move |cx, args| {
                let (cx, args) = hold_and_release!(cx, args);
                fetch::bless_fetch_request(hold!(cx.clone(), args)).map_err(|e| to_js_error(cx, e))
            }),
        )?,
    )?;
    ctx.eval::<(), _>(include_str!("fetch/fetch.js"))?;
    Ok(())
}

#[cfg(feature = "wasip1")]
pub fn set_wasi_globals(ctx: &Ctx<'_>) -> Result<()> {
    macro_rules! bind {
        (function, $l: ident) => {
            let name = concat!("__javy_", stringify!($l));
            ctx.globals().set(
                name,
                Function::new(
                    ctx.clone(),
                    MutFn::new(move |cx, args| {
                        let (cx, args) = hold_and_release!(cx, args);
                        wasi::$l(hold!(cx.clone(), args)).map_err(|e| to_js_error(cx, e))
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
    bind!(function, wasi_preview1_path_rename);
    bind!(function, wasi_preview1_path_filestat_get);
    ctx.eval::<(), _>(include_str!("wasi/preview_1.js"))?;
    Ok(())
}

#[cfg(feature = "llm")]
pub fn set_llm_globals(ctx: &Ctx<'_>) -> Result<()> {
    ctx.globals().set(
        "BlessLLM",
        Function::new(
            ctx.clone(),
            MutFn::new(move |cx, args| {
                let (cx, args) = hold_and_release!(cx, args);
                llm::bless_llm_plugin(hold!(cx.clone(), args)).map_err(|e| to_js_error(cx, e))
            }),
        )?,
    )?;

    // Expose the suppported models object globally for JS
    let ctx_clone = ctx.clone();
    ctx.globals().set(
        "MODELS",
        javy_plugin_api::javy::quickjs::Value::from_object(
            llm::supported_models_object(ctx).map_err(|e| to_js_error(ctx_clone, e))?,
        ),
    )?;
    Ok(())
}

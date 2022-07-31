use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use deno_runtime::{
    deno_core::{error::AnyError, resolve_path, FsModuleLoader, ModuleSpecifier},
    permissions::{Permissions, PermissionsOptions},
    worker::{MainWorker, WorkerOptions},
    BootstrapOptions,
};

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let main_module = resolve_path("example.js")?;

    let allowlist = vec![PathBuf::from("log.txt")];
    let permissions = Permissions::from_options(&PermissionsOptions {
        allow_read: Some(allowlist.clone()),
        allow_write: Some(allowlist),
        ..Default::default()
    });

    let mut worker = create_main_worker(main_module.clone(), permissions);
    worker.execute_main_module(&main_module).await
}

fn create_main_worker(main_module: ModuleSpecifier, permissions: Permissions) -> MainWorker {
    // WorkerOptions has no default yet, which may change:
    // https://github.com/denoland/deno/pull/14860
    let options = WorkerOptions {
        bootstrap: BootstrapOptions {
            args: vec![],
            cpu_count: 1,
            debug_flag: false,
            enable_testing_features: false,
            location: Some(main_module.clone()),
            no_color: false,
            is_tty: true,
            runtime_version: "n/a".to_string(),
            ts_version: "n/a".to_string(),
            unstable: false,
            user_agent: "Deno".to_string(),
        },
        extensions: vec![],
        unsafely_ignore_certificate_errors: None,
        root_cert_store: None,
        seed: None,
        source_map_getter: None,
        format_js_error_fn: None,
        web_worker_preload_module_cb: Arc::new(|_| unreachable!()),
        create_web_worker_cb: Arc::new(|_| unreachable!()),
        maybe_inspector_server: None,
        should_break_on_first_statement: false,
        module_loader: Rc::new(FsModuleLoader),
        get_error_class_fn: None,
        origin_storage_dir: None,
        blob_store: Default::default(),
        broadcast_channel: Default::default(),
        shared_array_buffer_store: None,
        compiled_wasm_module_store: None,
        stdio: Default::default(),
    };

    MainWorker::bootstrap_from_options(main_module, permissions, options)
}

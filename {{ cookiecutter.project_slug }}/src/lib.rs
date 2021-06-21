use log::{debug, error, info, trace, warn};
use log4rs::file::Deserializers;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Once;
use tera::Context;
use tera::Tera;

#[macro_use]
extern crate serde_derive;

static INIT: Once = Once::new();

#[pyfunction]
fn init_log(log4rs_config: log4rs::file::RawConfig) -> Result<(), anyhow::Error> {
    let (appenders, _) = log4rs_config.appenders_lossy(&Deserializers::default());

    let (config, _) = log4rs::config::Config::builder()
        .appenders(appenders)
        .loggers(log4rs_config.loggers())
        .build_lossy(log4rs_config.root());

    let log4rs_logger = log4rs::Logger::new(config);

    let logger = Box::new(log4rs_logger);
    log::set_max_level(log::LevelFilter::Info);
    log::set_boxed_logger(logger)?;
    Ok(())
}

#[pyfunction]
pub fn log_config_from_project_root(config_file_path: String) {
    INIT.call_once(|| {
        let file_path = Path::new(&config_file_path);
        if cfg!(test) {
            log4rs::init_file(
                file_path.join("config/test/log4rs.yaml"),
                Default::default(),
            )
            .unwrap();
        } else if cfg!(debug_assertions) {
            log4rs::init_file(
                file_path.join("config/debug/log4rs.yaml"),
                Default::default(),
            )
            .unwrap();
        } else {
            log4rs::init_file(
                file_path.join("config/release/log4rs.yaml"),
                Default::default(),
            )
            .unwrap();
        }
    });
}

#[pyfunction]
pub fn log_config_file(config_file_path: String, log_file_name: Option<String>) -> PyResult<()> {
    let log_file_name = if log_file_name.is_none() {
        "{{ cookiecutter.project_slug }}_default.log".to_string()
    } else {
        log_file_name.unwrap()
    };

    let mut file = File::open(config_file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut tera = Tera::default();
    tera.add_raw_template("log4rs.config", &contents).unwrap();
    let mut context = Context::new();
    context.insert("log_file_name", &log_file_name);
    let result = tera.render("log4rs.config", &context);
    let log4rs_config = match result {
        Ok(r) => r.to_string(),
        Err(e) => {
            return Err(exceptions::PyRuntimeError::new_err(format!(
                "Parse template failed {:?}",
                e
            )));
        }
    };

    let log4rs_config: Result<log4rs::file::RawConfig, serde_yaml::Error> =
        serde_yaml::from_str(&log4rs_config);
    let log4rs_config = match log4rs_config {
        Ok(r) => r,
        Err(e) => {
            return Err(exceptions::PyRuntimeError::new_err(format!(
                "Log4rs setting is wrong {:?}",
                e
            )));
        }
    };

    INIT.call_once(|| {
        init_log(log4rs_config).unwrap();
    });
    Ok(())
}


#[pymodule]
fn rust_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(log_config_file))?;
    m.add_wrapped(wrap_pyfunction!(log_config_from_project_root))?;
    Ok(())
}

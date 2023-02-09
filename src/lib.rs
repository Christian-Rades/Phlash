#![cfg_attr(windows, feature(abi_vectorcall))]
use anyhow::{anyhow, Result};
use ext_php_rs::prelude::*;

use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};

#[php_function]
pub fn hello_world(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[php_function]
pub fn transform(styles: &str) -> Result<String> {
    let mut css =
        StyleSheet::parse(styles, ParserOptions::default()).map_err(|err| anyhow!("{}", err))?;

    css.minify(MinifyOptions::default())?;

    let mut config = PrinterOptions::default();
    config.minify = true;

    Ok(css.to_css(config)?.code)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

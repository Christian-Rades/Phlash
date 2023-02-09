#![cfg_attr(windows, feature(abi_vectorcall))]
use std::collections::HashSet;

use anyhow::{anyhow, Result};
use ext_php_rs::prelude::*;

use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use lightningcss::targets::Browsers;

#[php_class(name = "Phlash\\CssTransformer")]
struct CssTransformer {
    browsers: Browsers,
    minify: bool,
}

#[php_impl]
impl CssTransformer {
    pub fn __construct() -> Self {
        CssTransformer {
            browsers: Browsers::default(),
            minify: true,
        }
    }
    pub fn transform(&self, styles: &str) -> Result<String> {
        let mut css = StyleSheet::parse(styles, ParserOptions::default())
            .map_err(|err| anyhow!("{}", err))?;

        let minify_conf = MinifyOptions {
            targets: Some(self.browsers.clone()),
            unused_symbols: HashSet::default(),
        };

        css.minify(minify_conf)?;

        let mut config = PrinterOptions::default();
        config.minify = self.minify;

        Ok(css.to_css(config)?.code)
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

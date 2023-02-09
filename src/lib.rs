#![cfg_attr(windows, feature(abi_vectorcall))]
use std::collections::HashSet;

use anyhow::{anyhow, Result};
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};

use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use lightningcss::targets::Browsers;

#[php_class(name = "Phlash\\CssTransformer")]
struct CssTransformer {
    browsers: Browsers,
    minify: bool,
}

#[php_impl]
impl CssTransformer {
    #[optional(config)]
    pub fn __construct(config: Option<&ZendHashTable>) -> Result<Self> {
        let mut base = CssTransformer {
            browsers: Browsers::default(),
            minify: true,
        };

        let Some(config_ht) = config else {
            return Ok(base);
        };

        for (_idx, key, val) in config_ht.iter() {
            let Some(key) = key else {continue};

            match key.as_ref() {
                "targets" => {
                    let map = val.array().ok_or(anyhow!("targets must be an array"))?;
                    if let Some(version) = map.get("chrome").and_then(Zval::str) {
                        base.browsers.chrome = Some(semver_to_u32(version)?);
                    }
                }
                "minify" => {
                    base.minify = val.bool().unwrap_or_default();
                }
                _ => {}
            }
        }

        return Ok(base);
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

fn semver_to_u32(version_str: &str) -> Result<u32> {
    let mut offset = 16;
    let mut version: u32 = 0;

    for part in version_str.split('.') {
        let part: u8 = part.parse()?;

        version = version | ((part as u32) << offset);

        if offset >= 8 {
            offset -= 8;
        }
    }
    Ok(version)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

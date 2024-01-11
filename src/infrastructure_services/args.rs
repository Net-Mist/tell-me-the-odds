use itertools::Itertools;

use anyhow::anyhow;
use anyhow::Result;
use std::env;

pub fn parse_cli() -> Result<(String, String)> {
    if let Some((millennium_data_path, empire_data_path)) = env::args().skip(1).collect_tuple() {
        Ok((millennium_data_path, empire_data_path))
    } else {
        Err(anyhow!(
            "script should have 2 arguments, millennium_data_path and empire_data_path",
        ))
    }
}

pub fn parse_webserver() -> Result<String> {
    if let Some((millennium_data_path,)) = env::args().skip(1).collect_tuple() {
        Ok(millennium_data_path)
    } else {
        Err(anyhow!(
            "script should have 1 argument, millennium_data_path",
        ))
    }
}

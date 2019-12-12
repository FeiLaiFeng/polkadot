// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use crate::ChainSpec;
use log::info;
use substrate_service::Configuration;
use wasm_bindgen::prelude::*;
use service::CustomConfiguration;

/// Starts the client.
///
/// You must pass a libp2p transport that supports .
#[wasm_bindgen]
pub async fn start_client(wasm_ext: sc_browser::Transport) -> Result<sc_browser::Client, JsValue> {
	start_inner(wasm_ext)
		.await
		.map_err(|err| JsValue::from_str(&err.to_string()))
}

async fn start_inner(wasm_ext: sc_browser::Transport) -> Result<sc_browser::Client, Box<dyn std::error::Error>> {
	sc_browser::set_hooks(log::Level::Info);

	let chain_spec = ChainSpec::Kusama.load().map_err(|e| format!("{:?}", e))?;
	let config: Configuration<CustomConfiguration, _, _> = sc_browser::browser_configuration(wasm_ext, chain_spec)
		.await?;

	info!("Polkadot browser node");
	info!("  version {}", config.full_version());
	info!("  by Parity Technologies, 2017-2019");
	info!("Chain specification: {}", config.chain_spec.name());
	if config.chain_spec.name().starts_with("Kusama") {
		info!("----------------------------");
		info!("This chain is not in any way");
		info!("      endorsed by the       ");
		info!("     KUSAMA FOUNDATION      ");
		info!("----------------------------");
	}
	info!("Node name: {}", config.name);
	info!("Roles: {:?}", config.roles);

	// Create the service. This is the most heavy initialization step.
	let service = service::new_light(config).map_err(|e| format!("{:?}", e))?;

	Ok(sc_browser::start_client(service))
}

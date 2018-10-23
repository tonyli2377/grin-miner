// Copyright 2017 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Tests exercising the loading and unloading of plugins, as well as the
/// existence and correct functionality of each plugin function
mod common;

extern crate rand;
extern crate cuckoo_miner as cuckoo;

use cuckoo::{PluginConfig};

#[test]
fn on_commit_mine_cuckatoo_mean_compat_cpu_19() {
	let mut config = PluginConfig::new("cuckatoo_mean_compat_cpu_19");
	config.params.nthreads = 4;
	common::mine_async_for_duration(&vec![config], 20);
}

#[test]
fn on_commit_mine_cuckatoo_mean_compat_cpu_29() {
	let mut config = PluginConfig::new("cuckatoo_mean_compat_cpu_29");
	config.params.nthreads = 4;
	common::mine_async_for_duration(&vec![config], 20);
}

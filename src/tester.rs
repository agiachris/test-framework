// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::callback::*;
use crate::expect_interface::*;
use crate::expectations::ExpectHandle;
use crate::host_settings::HostHandle;
use crate::hostcalls::{generate_import_list, get_abi_version};
use crate::settings_interface::*;
use crate::types::*;

use anyhow::Result;
use std::boxed::Box;
use std::sync::{Arc, Mutex, MutexGuard};
use wasmtime::*;

pub fn test(wasm_file: &str) -> Result<Box<dyn CallbackBase>> {
    // initialize wasm engine and shared cache
    let store = Store::default();
    let module = Module::from_file(store.engine(), wasm_file)?;

    // generate and link host function implementations
    let abi_version = get_abi_version(&module);
    let imports: Arc<Mutex<Vec<Extern>>> = Arc::new(Mutex::new(Vec::new()));
    let (host_settings, expectations): (Arc<Mutex<HostHandle>>, Arc<Mutex<ExpectHandle>>) =
        generate_import_list(&store, &module, imports.clone());
    let instance = Instance::new(&store, &module, &(*imports).lock().unwrap()[..])?;

    // create mock test proxy-wasm object
    impl CallbackBase for Tester {};
    match abi_version {
        AbiVersion::ProxyAbiVersion0_1_0 => {
            impl CallbackV1 for Tester {};
        }
        AbiVersion::ProxyAbiVersion0_2_0 => {
            impl CallbackV2 for Tester {};
        }
        _ => panic!("test-framework does not support proxy-wasm modules of this abi version"),
    }

    let tester = Tester::new(abi_version, instance, host_settings, expectations);
    return Ok(tester);
}

pub struct Tester {
    abi_version: AbiVersion,
    instance: Instance,
    defaults: Arc<Mutex<HostHandle>>,
    expect: Arc<Mutex<ExpectHandle>>,
}

impl Tester {
    fn new(
        abi_version: AbiVersion,
        instance: Instance,
        host_settings: Arc<Mutex<HostHandle>>,
        expect: Arc<Mutex<ExpectHandle>>,
    ) -> Tester {
        Tester {
            abi_version: abi_version,
            instance: instance,
            defaults: host_settings,
            expect: expect,
        }
    }

    pub fn execute_and_expect(&mut self, expect_wasm: ReturnType) -> Result<()> {
        let callback_result = execute_and_expect(&self.instance, expect_wasm);
        self.assert_expect_stage();
        self.update_expect_stage();
        println!("\n");
        callback_result
    }

    /* ------------------------------------- Low-level Expectation Setting ------------------------------------- */

    pub fn expect_log(&mut self, log_level: LogLevel, log_msg: &str) -> &mut Self {
        self.get_expect_handle()
            .staged
            .set_expect_log(log_level as i32, log_msg);
        self
    }

    pub fn expect_set_tick_period_millis(&mut self, tick_period_millis: u64) -> &mut Self {
        self.get_expect_handle()
            .staged
            .set_expect_set_tick_period_millis(tick_period_millis);
        self
    }

    pub fn expect_get_current_time_nanos(&mut self) -> ExpectGetCurrentTimeNanos {
        ExpectGetCurrentTimeNanos::expecting(self)
    }

    pub fn expect_get_buffer_bytes(&mut self, buffer_type: BufferType) -> ExpectGetBufferBytes {
        ExpectGetBufferBytes::expecting(self, buffer_type as i32)
    }

    pub fn expect_set_buffer_bytes(
        &mut self,
        buffer_type: BufferType,
        buffer_data: &str,
    ) -> &mut Self {
        self.get_expect_handle()
            .staged
            .set_expect_set_buffer_bytes(buffer_type as i32, buffer_data);
        self
    }

    pub fn expect_get_header_map_pairs(&mut self, map_type: MapType) -> ExpectGetHeaderMapPairs {
        ExpectGetHeaderMapPairs::expecting(self, map_type as i32)
    }

    pub fn expect_set_header_map_pairs(&mut self, map_type: MapType) -> ExpectSetHeaderMapPairs {
        ExpectSetHeaderMapPairs::expecting(self, map_type as i32)
    }

    pub fn expect_get_header_map_value(
        &mut self,
        map_type: MapType,
        header_map_key: &'static str,
    ) -> ExpectGetHeaderMapValue {
        ExpectGetHeaderMapValue::expecting(self, map_type as i32, header_map_key)
    }

    pub fn expect_replace_header_map_value(
        &mut self,
        map_type: MapType,
        header_map_key: &str,
        header_map_value: &str,
    ) -> &mut Self {
        self.get_expect_handle()
            .staged
            .set_expect_replace_header_map_value(map_type as i32, header_map_key, header_map_value);
        self
    }

    pub fn expect_remove_header_map_value(
        &mut self,
        map_type: MapType,
        header_map_key: &str,
    ) -> &mut Self {
        self.get_expect_handle()
            .staged
            .set_expect_remove_header_map_value(map_type as i32, header_map_key);
        self
    }

    pub fn expect_add_header_map_value(
        &mut self,
        map_type: MapType,
        header_map_key: &str,
        header_map_value: &str,
    ) -> &mut Self {
        self.get_expect_handle()
            .staged
            .set_expect_add_header_map_value(map_type as i32, header_map_key, header_map_value);
        self
    }

    pub fn expect_send_local_response(
        &mut self,
        status_code: i32,
        body: Option<&str>,
        headers: Vec<(&str, &str)>,
        grpc_status: i32,
    ) -> &mut Self {
        self.get_expect_handle()
            .staged
            .set_expect_send_local_response(status_code, body, headers, grpc_status);
        self
    }

    pub fn expect_http_call(
        &mut self,
        upstream: &'static str,
        headers: Vec<(&'static str, &'static str)>,
        body: Option<&'static str>,
        trailers: Vec<(&'static str, &'static str)>,
        timeout: u64,
    ) -> ExpectHttpCall {
        ExpectHttpCall::expecting(self, upstream, headers, body, trailers, timeout)
    }

    /* ------------------------------------- High-level Expectation Setting ------------------------------------- */

    pub fn reset_default_tick_period_millis(&mut self) -> &mut Self {
        self.get_settings_handle().staged.reset_tick_period_millis();
        self
    }

    pub fn set_default_tick_period_millis(&mut self, tick_period_millis: u64) -> &mut Self {
        self.get_settings_handle()
            .staged
            .set_tick_period_millis(tick_period_millis);
        self
    }

    pub fn reset_default_buffer_bytes(&mut self) -> &mut Self {
        self.get_settings_handle().staged.reset_buffer_bytes();
        self
    }

    pub fn set_default_buffer_bytes(&mut self, buffer_type: BufferType) -> DefaultBufferBytes {
        DefaultBufferBytes::expecting(self, buffer_type as i32)
    }

    pub fn reset_default_header_map_pairs(&mut self) -> &mut Self {
        self.get_settings_handle().staged.reset_header_map_pairs();
        self
    }

    pub fn set_default_header_map_pairs(&mut self, map_type: MapType) -> DefaultHeaderMapPairs {
        DefaultHeaderMapPairs::expecting(self, map_type as i32)
    }

    /* ------------------------------------- Utility Functions ------------------------------------- */

    pub fn get_expect_handle(&self) -> MutexGuard<ExpectHandle> {
        self.expect.lock().unwrap()
    }

    pub fn print_expectations(&self) {
        self.expect.lock().unwrap().print_staged();
    }

    fn update_expect_stage(&mut self) {
        self.expect.lock().unwrap().update_stage();
    }

    fn assert_expect_stage(&mut self) {
        self.expect.lock().unwrap().assert_stage();
    }

    pub fn get_settings_handle(&self) -> MutexGuard<HostHandle> {
        self.defaults.lock().unwrap()
    }

    pub fn print_host_settings(&self) {
        self.defaults.lock().unwrap().print_staged();
    }

    pub fn reset_host_settings(&mut self) {
        self.defaults.lock().unwrap().reset(self.abi_version);
    }
}

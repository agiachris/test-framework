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
use std::sync::{Arc, Mutex, MutexGuard};
use wasmtime::*;

pub fn test(wasm_file: &str) -> Result<Tester> {
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
    }

    let tester = Tester::new(abi_version, instance, host_settings, expectations);
    return Ok(tester);
}

pub struct Tester {
    abi_version: AbiVersion,
    instance: Instance,
    defaults: Arc<Mutex<HostHandle>>,
    expect: Arc<Mutex<ExpectHandle>>,
    callback: CallbackType,
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
            callback: CallbackType::new(),
        }
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

    /* ------------------------------------- Wasm Function Executation ------------------------------------- */

    pub fn execute_and_expect(&mut self, expect_wasm: ReturnType) -> Result<()> {
        assert_ne!(self.function_call, FunctionCall::FunctionNotSet);
        assert_ne!(self.function_type, FunctionType::ReturnNotSet);

        let mut return_wasm: Option<i32> = None;
        match self.function_call {
            FunctionCall::Start() => {
                let _start = self
                    .instance
                    .get_func("_start")
                    .ok_or(anyhow::format_err!(
                        "failed to find `_start` function export"
                    ))?
                    .get0::<()>()?;
                _start()?;
            }

            FunctionCall::ProxyOnContextCreate(root_context_id, parent_context_id) => {
                let proxy_on_context_create = self
                    .instance
                    .get_func("proxy_on_context_create")
                    .ok_or(anyhow::format_err!(
                        "failed to find `proxy_on_context_create` function export"
                    ))?
                    .get2::<i32, i32, ()>()?;
                proxy_on_context_create(root_context_id, parent_context_id)?;
            }

            FunctionCall::ProxyOnDone(context_id) => {
                let proxy_on_done = self
                    .instance
                    .get_func("proxy_on_done")
                    .ok_or(anyhow::format_err!(
                        "failed to find 'proxy_on_done' function export"
                    ))?
                    .get1::<i32, i32>()?;
                let is_done = proxy_on_done(context_id)?;
                println!("RETURN:    is_done -> {}", is_done);
                return_wasm = Some(is_done);
            }

            FunctionCall::ProxyOnLog(context_id) => {
                let proxy_on_log = self
                    .instance
                    .get_func("proxy_on_log")
                    .ok_or(anyhow::format_err!(
                        "failed to find `proxy_on_log` function export"
                    ))?
                    .get1::<i32, ()>()?;
                proxy_on_log(context_id)?;
            }

            FunctionCall::ProxyOnDelete(context_id) => {
                let proxy_on_delete = self
                    .instance
                    .get_func("proxy_on_delete")
                    .ok_or(anyhow::format_err!(
                        "failed to find 'proxy_on_delete' function export"
                    ))?
                    .get1::<i32, ()>()?;
                proxy_on_delete(context_id)?;
            }

            FunctionCall::ProxyOnVmStart(context_id, vm_configuration_size) => {
                let proxy_on_vm_start = self
                    .instance
                    .get_func("proxy_on_vm_start")
                    .ok_or(anyhow::format_err!(
                        "failed to find `proxy_on_vm_start` function export"
                    ))?
                    .get2::<i32, i32, i32>()?;
                let success = proxy_on_vm_start(context_id, vm_configuration_size)?;
                println!("RETURN:    success -> {}", success);
                return_wasm = Some(success);
            }

            FunctionCall::ProxyOnConfigure(context_id, plugin_configuration_size) => {
                let proxy_on_configure = self
                    .instance
                    .get_func("proxy_on_configure")
                    .ok_or(anyhow::format_err!(
                        "failed to find 'proxy_on_configure' function export"
                    ))?
                    .get2::<i32, i32, i32>()?;
                let success = proxy_on_configure(context_id, plugin_configuration_size)?;
                println!("RETURN:    success -> {}", success);
                return_wasm = Some(success);
            }

            FunctionCall::ProxyOnTick(context_id) => {
                let proxy_on_tick = self
                    .instance
                    .get_func("proxy_on_tick")
                    .ok_or(anyhow::format_err!(
                        "failed to find `proxy_on_tick` function export"
                    ))?
                    .get1::<i32, ()>()?;
                proxy_on_tick(context_id)?;
            }

            FunctionCall::ProxyOnQueueReady(context_id, queue_id) => {
                let proxy_on_queue_ready = self
                    .instance
                    .get_func("proxy_on_queue_ready")
                    .ok_or(anyhow::format_err!(
                        "failed to find 'proxy_on_queue_ready' function export"
                    ))?
                    .get2::<i32, i32, ()>()?;
                proxy_on_queue_ready(context_id, queue_id)?;
            }

            FunctionCall::ProxyOnNewConnection(context_id) => {
                let proxy_on_new_connection = self
                    .instance
                    .get_func("proxy_on_new_connection")
                    .ok_or(anyhow::format_err!(
                        "failed to find 'proxy_on_new_connection' function export"
                    ))?
                    .get1::<i32, i32>()?;
                let action = proxy_on_new_connection(context_id)?;
                println!("RETURN:    action -> {}", action);
                return_wasm = Some(action);
            }

            FunctionCall::ProxyOnDownstreamData(context_id, data_size, end_of_stream) => {
                let proxy_on_downstream_data = self
                    .instance
                    .get_func("proxy_on_downstream_data")
                    .ok_or(anyhow::format_err!(
                        "failed to find 'proxy_on_downstream_data' function export"
                    ))?
                    .get3::<i32, i32, i32, i32>()?;
                let action = proxy_on_downstream_data(context_id, data_size, end_of_stream)?;
                println!("RETURN:    action -> {}", action);
                return_wasm = Some(action);
            }

            FunctionCall::ProxyOnDownstreamConnectionClose(context_id, peer_type) => {
                let proxy_on_downstream_connection_close = self
                    .instance
                    .get_func("proxy_on_downstream_connection_close")
                    .ok_or(anyhow::format_err!(
                        "failed to find 'proxy_on_downstream_connection_close' function export"
                    ))?
                    .get2::<i32, i32, ()>()?;
                proxy_on_downstream_connection_close(context_id, peer_type)?;
            }

            FunctionCall::ProxyOnUpstreamData(context_id, data_size, end_of_stream) => {
                let proxy_on_upstream_data = self
                    .instance
                    .get_func("proxy_on_upstream_data")
                    .ok_or(anyhow::format_err!(
                        "failed to find 'proxy_on_upstream_data' function export"
                    ))?
                    .get3::<i32, i32, i32, i32>()?;
                let action = proxy_on_upstream_data(context_id, data_size, end_of_stream)?;
                println!("RETURN:    action -> {}", action);
                return_wasm = Some(action);
            }

            FunctionCall::ProxyOnUpstreamConnectionClose(context_id, peer_type) => {
                let proxy_on_upstream_connection_close = self
                    .instance
                    .get_func("proxy_on_upstream_connection_close")
                    .ok_or(anyhow::format_err!(
                        "failed to find 'proxy_on_upstream_connection_close' function export"
                    ))?
                    .get2::<i32, i32, ()>()?;
                proxy_on_upstream_connection_close(context_id, peer_type)?;
            }

            FunctionCall::ProxyOnRequestHeaders(context_id, num_headers) => {
                let proxy_on_request_headers = self
                    .instance
                    .get_func("proxy_on_request_headers")
                    .ok_or(anyhow::format_err!(
                        "failed to find `proxy_on_request_headers` function export"
                    ))?
                    .get2::<i32, i32, i32>()?;
                let action = proxy_on_request_headers(context_id, num_headers)?;
                println!("RETURN:    action -> {}", action);
                return_wasm = Some(action);
            }

            FunctionCall::ProxyOnRequestBody(context_id, body_size, end_of_stream) => {
                let proxy_on_request_body = self
                    .instance
                    .get_func("proxy_on_request_body")
                    .ok_or(anyhow::format_err!(
                        "failed to find 'proxy_on_request_body' function export"
                    ))?
                    .get3::<i32, i32, i32, i32>()?;
                let action = proxy_on_request_body(context_id, body_size, end_of_stream)?;
                println!("RETURN:    action -> {}", action);
                return_wasm = Some(action);
            }

            FunctionCall::ProxyOnRequestTrailers(context_id, num_trailers) => {
                let proxy_on_request_trailers = self
                    .instance
                    .get_func("proxy_on_request_trailers")
                    .ok_or(anyhow::format_err!(
                        "failed to find `proxy_on_request_trailers` function export"
                    ))?
                    .get2::<i32, i32, i32>()?;
                let action = proxy_on_request_trailers(context_id, num_trailers)?;
                println!("RETURN:    action -> {}", action);
                return_wasm = Some(action);
            }

            FunctionCall::ProxyOnResponseHeaders(context_id, num_headers) => {
                let proxy_on_response_headers = self
                    .instance
                    .get_func("proxy_on_response_headers")
                    .ok_or(anyhow::format_err!(
                        "failed to find `proxy_on_response_headers` function export"
                    ))?
                    .get2::<i32, i32, i32>()?;
                let action = proxy_on_response_headers(context_id, num_headers)?;
                println!("RETURN:    action -> {}", action);
                return_wasm = Some(action);
            }

            FunctionCall::ProxyOnResponseBody(context_id, body_size, end_of_stream) => {
                let proxy_on_response_body = self
                    .instance
                    .get_func("proxy_on_response_body")
                    .ok_or(anyhow::format_err!(
                        "failed to find 'proxy_on_response_body' function export"
                    ))?
                    .get3::<i32, i32, i32, i32>()?;
                let action = proxy_on_response_body(context_id, body_size, end_of_stream)?;
                println!("RETURN:    action -> {}", action);
                return_wasm = Some(action);
            }

            FunctionCall::ProxyOnResponseTrailers(context_id, num_trailers) => {
                let proxy_on_response_trailers = self
                    .instance
                    .get_func("proxy_on_response_trailers")
                    .ok_or(anyhow::format_err!(
                        "failed to find `proxy_on_response_trailers` function export"
                    ))?
                    .get2::<i32, i32, i32>()?;
                let action = proxy_on_response_trailers(context_id, num_trailers)?;
                println!("RETURN:    action -> {}", action);
                return_wasm = Some(action);
            }

            FunctionCall::ProxyOnHttpCallResponse(
                context_id,
                callout_id,
                num_headers,
                body_size,
                num_trailers,
            ) => {
                let proxy_on_http_call_response = self
                    .instance
                    .get_func("proxy_on_http_call_response")
                    .ok_or(anyhow::format_err!(
                        "failed to find `proxy_on_http_call_response` function export"
                    ))?
                    .get5::<i32, i32, i32, i32, i32, ()>()?;
                proxy_on_http_call_response(
                    context_id,
                    callout_id,
                    num_headers,
                    body_size,
                    num_trailers,
                )?;
            }

            _ => panic!("No function with name: {:?}", self.function_call),
        }

        match expect_wasm {
            ReturnType::None => {
                assert_eq!(self.function_type, FunctionType::ReturnEmpty);
                assert_eq!(return_wasm.is_none(), true);
            }
            ReturnType::Bool(expect_bool) => {
                assert_eq!(self.function_type, FunctionType::ReturnBool);
                assert_eq!(expect_bool as i32, return_wasm.unwrap_or(-1));
            }
            ReturnType::Action(expect_action) => {
                assert_eq!(self.function_type, FunctionType::ReturnAction);
                assert_eq!(expect_action as i32, return_wasm.unwrap_or(-1))
            }
        }

        self.function_call = FunctionCall::FunctionNotSet;
        self.function_type = FunctionType::ReturnNotSet;
        self.assert_expect_stage();
        self.update_expect_stage();
        println!("\n");
        return Ok(());
    }
}

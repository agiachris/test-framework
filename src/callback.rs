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

use crate::types::*;

use anyhow::Result;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use wasmtime::Instance;

lazy_static! {
    static ref CALLBACK: Arc<Mutex<CallbackType>> = Arc::new(Mutex::new(CallbackType::new()));
}

pub fn _clone_callback() -> Arc<Mutex<CallbackType>> {
    CALLBACK.clone()
}

fn set_callback(proto: CallbackProto, rtype: CallbackReturn) {
    CALLBACK.lock().unwrap().set(proto, rtype);
}

fn get_callback() -> (CallbackProto, CallbackReturn) {
    CALLBACK.lock().unwrap().get()
}

fn reset_callback() {
    CALLBACK.lock().unwrap().reset();
}

pub struct CallbackType(CallbackProto, CallbackReturn);
impl CallbackType {
    pub fn new() -> CallbackType {
        CallbackType(CallbackProto::FunctionNotSet, CallbackReturn::ReturnNotSet)
    }

    pub fn set(&mut self, proto: CallbackProto, rtype: CallbackReturn) {
        self.0 = proto;
        self.1 = rtype;
    }

    pub fn get(&self) -> (CallbackProto, CallbackReturn) {
        (self.0, self.1)
    }

    pub fn reset(&mut self) {
        self.0 = CallbackProto::FunctionNotSet;
        self.1 = CallbackReturn::ReturnNotSet;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CallbackProto {
    FunctionNotSet,
    Start(),
    ProxyOnContextCreate(i32, i32),
    ProxyOnLog(i32),
    ProxyOnDone(i32),
    ProxyOnForeignFunction(i32, i32, i32),
    ProxyOnDelete(i32),
    ProxyOnVmStart(i32, i32),
    ProxyOnConfigure(i32, i32),
    ProxyOnTick(i32),
    ProxyOnQueueReady(i32, i32),
    ProxyOnNewConnection(i32),
    ProxyOnDownstreamData(i32, i32, i32),
    ProxyOnDownstreamConnectionClose(i32, i32),
    ProxyOnUpstreamData(i32, i32, i32),
    ProxyOnUpstreamConnectionClose(i32, i32),
    ProxyOnRequestHeadersV1(i32, i32),
    ProxyOnRequestHeadersV2(i32, i32, i32),
    ProxyOnRequestBody(i32, i32, i32),
    ProxyOnRequestTrailers(i32, i32),
    ProxyOnResponseHeadersV1(i32, i32),
    ProxyOnResponseHeadersV2(i32, i32, i32),
    ProxyOnResponseBody(i32, i32, i32),
    ProxyOnResponseTrailers(i32, i32),
    ProxyOnHttpCallResponse(i32, i32, i32, i32, i32),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CallbackReturn {
    ReturnNotSet,
    ReturnEmpty,
    ReturnBool,
    ReturnAction,
}

pub trait CallbackBase {
    fn call_start(&mut self) -> &mut Self {
        println!("CALL TO:   _start");
        set_callback(CallbackProto::Start(), CallbackReturn::ReturnEmpty);
        self
    }

    fn call_proxy_on_context_create(
        &mut self,
        root_context_id: i32,
        parent_context_id: i32,
    ) -> &mut Self {
        println!("CALL TO:   proxy_on_context_create");
        println!(
            "ARGS:      root_context_id -> {}, parent_context_id -> {}",
            root_context_id, parent_context_id
        );
        set_callback(
            CallbackProto::ProxyOnContextCreate(root_context_id, parent_context_id),
            CallbackReturn::ReturnEmpty,
        );
        self
    }

    fn call_proxy_on_done(&mut self, context_id: i32) -> &mut Self {
        println!("CALL TO:   proxy_on_done");
        println!("ARGS:      context_id -> {}", context_id);
        set_callback(
            CallbackProto::ProxyOnDone(context_id),
            CallbackReturn::ReturnBool,
        );
        self
    }

    fn call_proxy_on_log(&mut self, context_id: i32) -> &mut Self {
        println!("CALL TO:   proxy_on_log");
        println!("ARGS:      context_id -> {}", context_id);
        set_callback(
            CallbackProto::ProxyOnLog(context_id),
            CallbackReturn::ReturnEmpty,
        );
        self
    }

    fn call_proxy_on_delete(&mut self, context_id: i32) -> &mut Self {
        println!("CALL TO:   proxy_on_delete");
        println!("ARGS:      context_id -> {}", context_id);
        set_callback(
            CallbackProto::ProxyOnDelete(context_id),
            CallbackReturn::ReturnEmpty,
        );
        self
    }

    fn call_proxy_on_vm_start(&mut self, context_id: i32, vm_configuration_size: i32) -> &mut Self {
        println!("CALL TO:   proxy_on_vm_start");
        println!(
            "ARGS:      context_id -> {}, vm_configuration_size -> {}",
            context_id, vm_configuration_size
        );
        set_callback(
            CallbackProto::ProxyOnVmStart(context_id, vm_configuration_size),
            CallbackReturn::ReturnBool,
        );
        self
    }

    fn call_proxy_on_configure(
        &mut self,
        context_id: i32,
        plugin_configuration_size: i32,
    ) -> &mut Self {
        println!("CALL TO:   proxy_on_configure");
        println!(
            "ARGS:      context_id -> {}, plugin_configuration_size -> {}",
            context_id, plugin_configuration_size
        );
        set_callback(
            CallbackProto::ProxyOnConfigure(context_id, plugin_configuration_size),
            CallbackReturn::ReturnBool,
        );
        self
    }

    fn call_proxy_on_tick(&mut self, context_id: i32) -> &mut Self {
        println!("CALL TO:   proxy_on_tick");
        println!("ARGS:      context_id -> {}", context_id);
        set_callback(
            CallbackProto::ProxyOnTick(context_id),
            CallbackReturn::ReturnEmpty,
        );
        self
    }

    fn call_proxy_on_queue_ready(&mut self, context_id: i32, queue_id: i32) -> &mut Self {
        println!("CALL TO:   proxy_on_queue_ready");
        println!(
            "ARGS:      context_id -> {}, queue_id -> {}",
            context_id, queue_id
        );
        set_callback(
            CallbackProto::ProxyOnQueueReady(context_id, queue_id),
            CallbackReturn::ReturnEmpty,
        );
        self
    }

    fn call_proxy_on_new_connection(&mut self, context_id: i32) -> &mut Self {
        println!("CALL TO:   proxy_on_new_connection");
        println!("ARGS:      context_id -> {}", context_id);
        set_callback(
            CallbackProto::ProxyOnNewConnection(context_id),
            CallbackReturn::ReturnAction,
        );
        self
    }

    fn call_proxy_on_downstream_data(
        &mut self,
        context_id: i32,
        data_size: i32,
        end_of_stream: i32,
    ) -> &mut Self {
        println!("CALL TO:   proxy_on_downstream_data");
        println!(
            "ARGS:      context_id -> {}, data_size -> {}, end_of_stream -> {}",
            context_id, data_size, end_of_stream
        );
        set_callback(
            CallbackProto::ProxyOnDownstreamData(context_id, data_size, end_of_stream),
            CallbackReturn::ReturnAction,
        );
        self
    }

    fn call_proxy_on_downstream_connection_close(
        &mut self,
        context_id: i32,
        peer_type: PeerType,
    ) -> &mut Self {
        println!("CALL TO:   proxy_on_downstream_connection_close");
        println!(
            "ARGS:      context_id -> {}, peer_data -> {}",
            context_id, peer_type as i32
        );
        set_callback(
            CallbackProto::ProxyOnDownstreamConnectionClose(context_id, peer_type as i32),
            CallbackReturn::ReturnEmpty,
        );
        self
    }

    fn call_proxy_on_upstream_data(
        &mut self,
        context_id: i32,
        data_size: i32,
        end_of_stream: i32,
    ) -> &mut Self {
        println!("CALL TO:   proxy_on_upstream_data");
        println!(
            "ARGS:      context_id -> {}, data_size -> {}, end_of_stream -> {}",
            context_id, data_size, end_of_stream
        );
        set_callback(
            CallbackProto::ProxyOnUpstreamData(context_id, data_size, end_of_stream),
            CallbackReturn::ReturnAction,
        );
        self
    }

    fn call_proxy_on_upstream_connection_close(
        &mut self,
        context_id: i32,
        peer_type: PeerType,
    ) -> &mut Self {
        println!("CALL TO:   proxy_on_upstream_connection_close");
        println!(
            "ARGS:      context_id -> {}, peer_data -> {}",
            context_id, peer_type as i32
        );
        set_callback(
            CallbackProto::ProxyOnUpstreamConnectionClose(context_id, peer_type as i32),
            CallbackReturn::ReturnEmpty,
        );
        self
    }

    fn call_proxy_on_request_body(
        &mut self,
        context_id: i32,
        body_size: i32,
        end_of_stream: i32,
    ) -> &mut Self {
        println!("CALL TO:   proxy_on_request_body");
        println!(
            "ARGS:      context_id -> {}, body_size -> {}, end_of_stream -> {}",
            context_id, body_size, end_of_stream
        );
        set_callback(
            CallbackProto::ProxyOnRequestBody(context_id, body_size, end_of_stream),
            CallbackReturn::ReturnAction,
        );
        self
    }

    fn call_proxy_on_request_trailers(&mut self, context_id: i32, num_trailers: i32) -> &mut Self {
        println!("CALL TO:   proxy_on_request_trailers");
        println!(
            "ARGS:      context_id -> {}, num_trailers -> {}",
            context_id, num_trailers
        );
        set_callback(
            CallbackProto::ProxyOnRequestTrailers(context_id, num_trailers),
            CallbackReturn::ReturnAction,
        );
        self
    }

    fn call_proxy_on_response_body(
        &mut self,
        context_id: i32,
        body_size: i32,
        end_of_stream: i32,
    ) -> &mut Self {
        println!("CALL TO:   proxy_on_response_body");
        println!(
            "ARGS:      context_id -> {}, body_size -> {}, end_of_stream -> {}",
            context_id, body_size, end_of_stream
        );
        set_callback(
            CallbackProto::ProxyOnResponseBody(context_id, body_size, end_of_stream),
            CallbackReturn::ReturnAction,
        );
        self
    }

    fn call_proxy_on_response_trailers(&mut self, context_id: i32, num_trailers: i32) -> &mut Self {
        println!("CALL TO:   proxy_on_response_trailers");
        println!(
            "ARGS:      context_id -> {}, num_trailers -> {}",
            context_id, num_trailers
        );
        set_callback(
            CallbackProto::ProxyOnResponseTrailers(context_id, num_trailers),
            CallbackReturn::ReturnAction,
        );
        self
    }

    fn call_proxy_on_http_call_response(
        &mut self,
        context_id: i32,
        callout_id: i32,
        num_headers: i32,
        body_size: i32,
        num_trailers: i32,
    ) -> &mut Self {
        println!("CALL TO:   proxy_on_http_call_response");
        println!(
            "ARGS:      context_id -> {}, callout_id -> {}",
            context_id, callout_id
        );
        println!(
            "           num_headers -> {}, body_size -> {}, num_trailers: {}",
            num_headers, body_size, num_trailers
        );
        set_callback(
            CallbackProto::ProxyOnHttpCallResponse(
                context_id,
                callout_id,
                num_headers,
                body_size,
                num_trailers,
            ),
            CallbackReturn::ReturnEmpty,
        );
        self
    }

    /* ---------------------------------- Combination Calls ---------------------------------- */
}

pub trait CallbackV1: CallbackBase {
    fn call_proxy_on_request_headers(&mut self, context_id: i32, num_headers: i32) -> &mut Self {
        println!("CALL TO:   proxy_on_request_headers");
        println!(
            "ARGS:      context_id -> {}, num_headers -> {}",
            context_id, num_headers
        );
        set_callback(
            CallbackProto::ProxyOnRequestHeadersV1(context_id, num_headers),
            CallbackReturn::ReturnAction,
        );
        self
    }

    fn call_proxy_on_response_headers(&mut self, context_id: i32, num_headers: i32) -> &mut Self {
        println!("CALL TO:   proxy_on_response_headers");
        println!(
            "ARGS:      context_id -> {}, num_headers -> {}",
            context_id, num_headers
        );
        set_callback(
            CallbackProto::ProxyOnResponseHeadersV1(context_id, num_headers),
            CallbackReturn::ReturnAction,
        );
        self
    }
}

pub trait CallbackV2: CallbackBase {
    fn call_proxy_on_request_headers(
        &mut self,
        context_id: i32,
        num_headers: i32,
        end_of_stream: i32,
    ) -> &mut Self {
        println!("CALL TO:   proxy_on_request_headers");
        println!(
            "ARGS:      context_id -> {}, num_headers -> {}, end_of_stream -> {}",
            context_id, num_headers, end_of_stream
        );
        set_callback(
            CallbackProto::ProxyOnRequestHeadersV2(context_id, num_headers, end_of_stream),
            CallbackReturn::ReturnAction,
        );
        self
    }

    fn call_proxy_on_response_headers(
        &mut self,
        context_id: i32,
        num_headers: i32,
        end_of_stream: i32,
    ) -> &mut Self {
        println!("CALL TO:   proxy_on_response_headers");
        println!(
            "ARGS:      context_id -> {}, num_headers -> {}, end_of_stream -> {}",
            context_id, num_headers, end_of_stream
        );
        set_callback(
            CallbackProto::ProxyOnRequestHeadersV2(context_id, num_headers, end_of_stream),
            CallbackReturn::ReturnAction,
        );
        self
    }

    fn call_proxy_on_foreign_function(
        &mut self,
        root_context_id: i32,
        function_id: i32,
        data_size: i32,
    ) -> &mut Self {
        set_callback(
            CallbackProto::ProxyOnForeignFunction(root_context_id, function_id, data_size),
            CallbackReturn::ReturnAction,
        );
        self
    }
}

/* ------------------------------------- Wasm Function Executation ------------------------------------- */

pub fn execute_and_expect(instance: &Instance, expect_wasm: ReturnType) -> Result<()> {
    let (callback_proto, callback_rtype) = get_callback();
    assert_ne!(callback_proto, CallbackProto::FunctionNotSet);
    assert_ne!(callback_rtype, CallbackReturn::ReturnNotSet);

    let mut return_wasm: Option<i32> = None;
    match callback_proto {
        CallbackProto::Start() => {
            let _start = instance
                .get_func("_start")
                .ok_or(anyhow::format_err!(
                    "failed to find `_start` function export"
                ))?
                .get0::<()>()?;
            _start()?;
        }

        CallbackProto::ProxyOnContextCreate(root_context_id, parent_context_id) => {
            let proxy_on_context_create = instance
                .get_func("proxy_on_context_create")
                .ok_or(anyhow::format_err!(
                    "failed to find `proxy_on_context_create` function export"
                ))?
                .get2::<i32, i32, ()>()?;
            proxy_on_context_create(root_context_id, parent_context_id)?;
        }

        CallbackProto::ProxyOnDone(context_id) => {
            let proxy_on_done = instance
                .get_func("proxy_on_done")
                .ok_or(anyhow::format_err!(
                    "failed to find 'proxy_on_done' function export"
                ))?
                .get1::<i32, i32>()?;
            let is_done = proxy_on_done(context_id)?;
            println!("RETURN:    is_done -> {}", is_done);
            return_wasm = Some(is_done);
        }

        CallbackProto::ProxyOnForeignFunction(root_context_id, function_id, data_size) => {
            let proxy_on_foreign_function = instance
                .get_func("proxy_on_foreign_function")
                .ok_or(anyhow::format_err!(
                    "failed to find 'proxy_on_foreign_function' function export"
                ))?
                .get3::<i32, i32, i32, i32>()?;
            let action = proxy_on_foreign_function(root_context_id, function_id, data_size)?;
            println!("RETURN:    action -> {}", action);
            return_wasm = Some(action);
        }

        CallbackProto::ProxyOnLog(context_id) => {
            let proxy_on_log = instance
                .get_func("proxy_on_log")
                .ok_or(anyhow::format_err!(
                    "failed to find `proxy_on_log` function export"
                ))?
                .get1::<i32, ()>()?;
            proxy_on_log(context_id)?;
        }

        CallbackProto::ProxyOnDelete(context_id) => {
            let proxy_on_delete = instance
                .get_func("proxy_on_delete")
                .ok_or(anyhow::format_err!(
                    "failed to find 'proxy_on_delete' function export"
                ))?
                .get1::<i32, ()>()?;
            proxy_on_delete(context_id)?;
        }

        CallbackProto::ProxyOnVmStart(context_id, vm_configuration_size) => {
            let proxy_on_vm_start = instance
                .get_func("proxy_on_vm_start")
                .ok_or(anyhow::format_err!(
                    "failed to find `proxy_on_vm_start` function export"
                ))?
                .get2::<i32, i32, i32>()?;
            let success = proxy_on_vm_start(context_id, vm_configuration_size)?;
            println!("RETURN:    success -> {}", success);
            return_wasm = Some(success);
        }

        CallbackProto::ProxyOnConfigure(context_id, plugin_configuration_size) => {
            let proxy_on_configure = instance
                .get_func("proxy_on_configure")
                .ok_or(anyhow::format_err!(
                    "failed to find 'proxy_on_configure' function export"
                ))?
                .get2::<i32, i32, i32>()?;
            let success = proxy_on_configure(context_id, plugin_configuration_size)?;
            println!("RETURN:    success -> {}", success);
            return_wasm = Some(success);
        }

        CallbackProto::ProxyOnTick(context_id) => {
            let proxy_on_tick = instance
                .get_func("proxy_on_tick")
                .ok_or(anyhow::format_err!(
                    "failed to find `proxy_on_tick` function export"
                ))?
                .get1::<i32, ()>()?;
            proxy_on_tick(context_id)?;
        }

        CallbackProto::ProxyOnQueueReady(context_id, queue_id) => {
            let proxy_on_queue_ready = instance
                .get_func("proxy_on_queue_ready")
                .ok_or(anyhow::format_err!(
                    "failed to find 'proxy_on_queue_ready' function export"
                ))?
                .get2::<i32, i32, ()>()?;
            proxy_on_queue_ready(context_id, queue_id)?;
        }

        CallbackProto::ProxyOnNewConnection(context_id) => {
            let proxy_on_new_connection = instance
                .get_func("proxy_on_new_connection")
                .ok_or(anyhow::format_err!(
                    "failed to find 'proxy_on_new_connection' function export"
                ))?
                .get1::<i32, i32>()?;
            let action = proxy_on_new_connection(context_id)?;
            println!("RETURN:    action -> {}", action);
            return_wasm = Some(action);
        }

        CallbackProto::ProxyOnDownstreamData(context_id, data_size, end_of_stream) => {
            let proxy_on_downstream_data = instance
                .get_func("proxy_on_downstream_data")
                .ok_or(anyhow::format_err!(
                    "failed to find 'proxy_on_downstream_data' function export"
                ))?
                .get3::<i32, i32, i32, i32>()?;
            let action = proxy_on_downstream_data(context_id, data_size, end_of_stream)?;
            println!("RETURN:    action -> {}", action);
            return_wasm = Some(action);
        }

        CallbackProto::ProxyOnDownstreamConnectionClose(context_id, peer_type) => {
            let proxy_on_downstream_connection_close = instance
                .get_func("proxy_on_downstream_connection_close")
                .ok_or(anyhow::format_err!(
                    "failed to find 'proxy_on_downstream_connection_close' function export"
                ))?
                .get2::<i32, i32, ()>()?;
            proxy_on_downstream_connection_close(context_id, peer_type)?;
        }

        CallbackProto::ProxyOnUpstreamData(context_id, data_size, end_of_stream) => {
            let proxy_on_upstream_data = instance
                .get_func("proxy_on_upstream_data")
                .ok_or(anyhow::format_err!(
                    "failed to find 'proxy_on_upstream_data' function export"
                ))?
                .get3::<i32, i32, i32, i32>()?;
            let action = proxy_on_upstream_data(context_id, data_size, end_of_stream)?;
            println!("RETURN:    action -> {}", action);
            return_wasm = Some(action);
        }

        CallbackProto::ProxyOnUpstreamConnectionClose(context_id, peer_type) => {
            let proxy_on_upstream_connection_close = instance
                .get_func("proxy_on_upstream_connection_close")
                .ok_or(anyhow::format_err!(
                    "failed to find 'proxy_on_upstream_connection_close' function export"
                ))?
                .get2::<i32, i32, ()>()?;
            proxy_on_upstream_connection_close(context_id, peer_type)?;
        }

        CallbackProto::ProxyOnRequestHeadersV1(context_id, num_headers) => {
            let proxy_on_request_headers = instance
                .get_func("proxy_on_request_headers")
                .ok_or(anyhow::format_err!(
                    "failed to find `proxy_on_request_headers` function export"
                ))?
                .get2::<i32, i32, i32>()?;
            let action = proxy_on_request_headers(context_id, num_headers)?;
            println!("RETURN:    action -> {}", action);
            return_wasm = Some(action);
        }

        CallbackProto::ProxyOnRequestHeadersV2(context_id, num_headers, end_of_stream) => {
            let proxy_on_request_headers = instance
                .get_func("proxy_on_request_headers")
                .ok_or(anyhow::format_err!(
                    "failed to find `proxy_on_request_headers` function export"
                ))?
                .get3::<i32, i32, i32, i32>()?;
            let action = proxy_on_request_headers(context_id, num_headers, end_of_stream)?;
            println!("RETURN:    action -> {}", action);
            return_wasm = Some(action);
        }

        CallbackProto::ProxyOnRequestBody(context_id, body_size, end_of_stream) => {
            let proxy_on_request_body = instance
                .get_func("proxy_on_request_body")
                .ok_or(anyhow::format_err!(
                    "failed to find 'proxy_on_request_body' function export"
                ))?
                .get3::<i32, i32, i32, i32>()?;
            let action = proxy_on_request_body(context_id, body_size, end_of_stream)?;
            println!("RETURN:    action -> {}", action);
            return_wasm = Some(action);
        }

        CallbackProto::ProxyOnRequestTrailers(context_id, num_trailers) => {
            let proxy_on_request_trailers = instance
                .get_func("proxy_on_request_trailers")
                .ok_or(anyhow::format_err!(
                    "failed to find `proxy_on_request_trailers` function export"
                ))?
                .get2::<i32, i32, i32>()?;
            let action = proxy_on_request_trailers(context_id, num_trailers)?;
            println!("RETURN:    action -> {}", action);
            return_wasm = Some(action);
        }

        CallbackProto::ProxyOnResponseHeadersV1(context_id, num_headers) => {
            let proxy_on_response_headers = instance
                .get_func("proxy_on_response_headers")
                .ok_or(anyhow::format_err!(
                    "failed to find `proxy_on_response_headers` function export"
                ))?
                .get2::<i32, i32, i32>()?;
            let action = proxy_on_response_headers(context_id, num_headers)?;
            println!("RETURN:    action -> {}", action);
            return_wasm = Some(action);
        }

        CallbackProto::ProxyOnResponseHeadersV2(context_id, num_headers, end_of_stream) => {
            let proxy_on_response_headers = instance
                .get_func("proxy_on_response_headers")
                .ok_or(anyhow::format_err!(
                    "failed to find `proxy_on_response_headers` function export"
                ))?
                .get3::<i32, i32, i32, i32>()?;
            let action = proxy_on_response_headers(context_id, num_headers, end_of_stream)?;
            println!("RETURN:    action -> {}", action);
            return_wasm = Some(action);
        }

        CallbackProto::ProxyOnResponseBody(context_id, body_size, end_of_stream) => {
            let proxy_on_response_body = instance
                .get_func("proxy_on_response_body")
                .ok_or(anyhow::format_err!(
                    "failed to find 'proxy_on_response_body' function export"
                ))?
                .get3::<i32, i32, i32, i32>()?;
            let action = proxy_on_response_body(context_id, body_size, end_of_stream)?;
            println!("RETURN:    action -> {}", action);
            return_wasm = Some(action);
        }

        CallbackProto::ProxyOnResponseTrailers(context_id, num_trailers) => {
            let proxy_on_response_trailers = instance
                .get_func("proxy_on_response_trailers")
                .ok_or(anyhow::format_err!(
                    "failed to find `proxy_on_response_trailers` function export"
                ))?
                .get2::<i32, i32, i32>()?;
            let action = proxy_on_response_trailers(context_id, num_trailers)?;
            println!("RETURN:    action -> {}", action);
            return_wasm = Some(action);
        }

        CallbackProto::ProxyOnHttpCallResponse(
            context_id,
            callout_id,
            num_headers,
            body_size,
            num_trailers,
        ) => {
            let proxy_on_http_call_response = instance
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

        _ => panic!("No function with name: {:?}", callback_proto),
    }

    match expect_wasm {
        ReturnType::None => {
            assert_eq!(callback_rtype, CallbackReturn::ReturnEmpty);
            assert_eq!(return_wasm.is_none(), true);
        }
        ReturnType::Bool(expect_bool) => {
            assert_eq!(callback_rtype, CallbackReturn::ReturnBool);
            assert_eq!(expect_bool as i32, return_wasm.unwrap_or(-1));
        }
        ReturnType::Action(expect_action) => {
            assert_eq!(callback_rtype, CallbackReturn::ReturnAction);
            assert_eq!(expect_action as i32, return_wasm.unwrap_or(-1))
        }
    }

    reset_callback();
    Ok(())
}

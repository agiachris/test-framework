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

// Callback interface structure for ABI version 0_1_0
pub struct CallbackInterfaceV1 {}
impl CallbackBase for CallbackInterfaceV1 {}
impl CallbackV1 for CallbackInterfaceV1 {}

// Callback interface structure for ABI version 0_2_0
pub struct CallbackInterfaceV2 {}
impl CallbackBase for CallbackInterfaceV2 {}
impl CallbackV2 for CallbackInterfaceV2 {}

pub struct CallbackType(CallbackProto, CallbackReturn);

#[derive(Debug, PartialEq)]
enum CallbackProto {
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

#[derive(Debug, PartialEq)]
enum CallbackReturn {
    ReturnNotSet,
    ReturnEmpty,
    ReturnBool,
    ReturnAction,
}

pub trait CallbackV1: CallbackBase {
    fn call_proxy_on_request_headers(&self, context_id: i32, num_headers: i32) -> CallbackType {
        println!("CALL TO:   proxy_on_request_headers");
        println!(
            "ARGS:      context_id -> {}, num_headers -> {}",
            context_id, num_headers
        );
        CallbackType(
            CallbackProto::ProxyOnRequestHeadersV1(context_id, num_headers),
            CallbackReturn::ReturnAction,
        )
    }

    fn call_proxy_on_response_headers(&self, context_id: i32, num_headers: i32) -> CallbackType {
        println!("CALL TO:   proxy_on_response_headers");
        println!(
            "ARGS:      context_id -> {}, num_headers -> {}",
            context_id, num_headers
        );
        CallbackType(
            CallbackProto::ProxyOnResponseHeadersV1(context_id, num_headers),
            CallbackReturn::ReturnAction,
        )
    }
}

pub trait CallbackV2: CallbackBase {
    fn call_proxy_on_request_headers(
        &self,
        context_id: i32,
        num_headers: i32,
        end_of_stream: i32,
    ) -> CallbackType {
        println!("CALL TO:   proxy_on_request_headers");
        println!(
            "ARGS:      context_id -> {}, num_headers -> {}, end_of_stream -> {}",
            context_id, num_headers, end_of_stream
        );
        CallbackType(
            CallbackProto::ProxyOnRequestHeadersV2(context_id, num_headers, end_of_stream),
            CallbackReturn::ReturnAction,
        )
    }

    fn call_proxy_on_response_headers(
        &self,
        context_id: i32,
        num_headers: i32,
        end_of_stream: i32,
    ) -> CallbackType {
        println!("CALL TO:   proxy_on_response_headers");
        println!(
            "ARGS:      context_id -> {}, num_headers -> {}, end_of_stream -> {}",
            context_id, num_headers, end_of_stream
        );
        CallbackType(
            CallbackProto::ProxyOnRequestHeadersV2(context_id, num_headers, end_of_stream),
            CallbackReturn::ReturnAction,
        )
    }

    fn call_proxy_on_foreign_function(
        &self,
        root_context_id: i32,
        function_id: i32,
        data_size: i32,
    ) -> CallbackType {
        CallbackType(
            CallbackProto::ProxyOnForeignFunction(root_context_id, function_id, data_size),
            CallbackReturn::ReturnAction,
        )
    }
}

pub trait CallbackBase {
    fn call_start(&self) -> CallbackType {
        println!("CALL TO:   _start");
        CallbackType(CallbackProto::Start(), CallbackReturn::ReturnEmpty)
    }

    fn call_proxy_on_context_create(
        &self,
        root_context_id: i32,
        parent_context_id: i32,
    ) -> CallbackType {
        println!("CALL TO:   proxy_on_context_create");
        println!(
            "ARGS:      root_context_id -> {}, parent_context_id -> {}",
            root_context_id, parent_context_id
        );
        CallbackType(
            CallbackProto::ProxyOnContextCreate(root_context_id, parent_context_id),
            CallbackReturn::ReturnEmpty,
        )
    }

    fn call_proxy_on_done(&self, context_id: i32) -> CallbackType {
        println!("CALL TO:   proxy_on_done");
        println!("ARGS:      context_id -> {}", context_id);
        CallbackType(
            CallbackProto::ProxyOnDone(context_id),
            CallbackReturn::ReturnBool,
        )
    }

    fn call_proxy_on_log(&self, context_id: i32) -> CallbackType {
        println!("CALL TO:   proxy_on_log");
        println!("ARGS:      context_id -> {}", context_id);
        CallbackType(
            CallbackProto::ProxyOnLog(context_id),
            CallbackReturn::ReturnEmpty,
        )
    }

    fn call_proxy_on_delete(&self, context_id: i32) -> CallbackType {
        println!("CALL TO:   proxy_on_delete");
        println!("ARGS:      context_id -> {}", context_id);
        CallbackType(
            CallbackProto::ProxyOnDelete(context_id),
            CallbackReturn::ReturnEmpty,
        )
    }

    fn call_proxy_on_vm_start(&self, context_id: i32, vm_configuration_size: i32) -> CallbackType {
        println!("CALL TO:   proxy_on_vm_start");
        println!(
            "ARGS:      context_id -> {}, vm_configuration_size -> {}",
            context_id, vm_configuration_size
        );
        CallbackType(
            CallbackProto::ProxyOnVmStart(context_id, vm_configuration_size),
            CallbackReturn::ReturnBool,
        )
    }

    fn call_proxy_on_configure(
        &self,
        context_id: i32,
        plugin_configuration_size: i32,
    ) -> CallbackType {
        println!("CALL TO:   proxy_on_configure");
        println!(
            "ARGS:      context_id -> {}, plugin_configuration_size -> {}",
            context_id, plugin_configuration_size
        );
        CallbackType(
            CallbackProto::ProxyOnConfigure(context_id, plugin_configuration_size),
            CallbackReturn::ReturnBool,
        )
    }

    fn call_proxy_on_tick(&self, context_id: i32) -> CallbackType {
        println!("CALL TO:   proxy_on_tick");
        println!("ARGS:      context_id -> {}", context_id);
        CallbackType(
            CallbackProto::ProxyOnTick(context_id),
            CallbackReturn::ReturnEmpty,
        )
    }

    fn call_proxy_on_queue_ready(&self, context_id: i32, queue_id: i32) -> CallbackType {
        println!("CALL TO:   proxy_on_queue_ready");
        println!(
            "ARGS:      context_id -> {}, queue_id -> {}",
            context_id, queue_id
        );
        CallbackType(
            CallbackProto::ProxyOnQueueReady(context_id, queue_id),
            CallbackReturn::ReturnEmpty,
        )
    }

    fn call_proxy_on_new_connection(&self, context_id: i32) -> CallbackType {
        println!("CALL TO:   proxy_on_new_connection");
        println!("ARGS:      context_id -> {}", context_id);
        CallbackType(
            CallbackProto::ProxyOnNewConnection(context_id),
            CallbackReturn::ReturnAction,
        )
    }

    fn call_proxy_on_downstream_data(
        &self,
        context_id: i32,
        data_size: i32,
        end_of_stream: i32,
    ) -> CallbackType {
        println!("CALL TO:   proxy_on_downstream_data");
        println!(
            "ARGS:      context_id -> {}, data_size -> {}, end_of_stream -> {}",
            context_id, data_size, end_of_stream
        );
        CallbackType(
            CallbackProto::ProxyOnDownstreamData(context_id, data_size, end_of_stream),
            CallbackReturn::ReturnAction,
        )
    }

    fn call_proxy_on_downstream_connection_close(
        &self,
        context_id: i32,
        peer_type: PeerType,
    ) -> CallbackType {
        println!("CALL TO:   proxy_on_downstream_connection_close");
        println!(
            "ARGS:      context_id -> {}, peer_data -> {}",
            context_id, peer_type as i32
        );
        CallbackType(
            CallbackProto::ProxyOnDownstreamConnectionClose(context_id, peer_type as i32),
            CallbackReturn::ReturnEmpty,
        )
    }

    fn call_proxy_on_upstream_data(
        &self,
        context_id: i32,
        data_size: i32,
        end_of_stream: i32,
    ) -> CallbackType {
        println!("CALL TO:   proxy_on_upstream_data");
        println!(
            "ARGS:      context_id -> {}, data_size -> {}, end_of_stream -> {}",
            context_id, data_size, end_of_stream
        );
        CallbackType(
            CallbackProto::ProxyOnUpstreamData(context_id, data_size, end_of_stream),
            CallbackReturn::ReturnAction,
        )
    }

    fn call_proxy_on_upstream_connection_close(
        &self,
        context_id: i32,
        peer_type: PeerType,
    ) -> CallbackType {
        println!("CALL TO:   proxy_on_upstream_connection_close");
        println!(
            "ARGS:      context_id -> {}, peer_data -> {}",
            context_id, peer_type as i32
        );
        CallbackType(
            CallbackProto::ProxyOnUpstreamConnectionClose(context_id, peer_type as i32),
            CallbackReturn::ReturnEmpty,
        )
    }

    fn call_proxy_on_request_body(
        &self,
        context_id: i32,
        body_size: i32,
        end_of_stream: i32,
    ) -> CallbackType {
        println!("CALL TO:   proxy_on_request_body");
        println!(
            "ARGS:      context_id -> {}, body_size -> {}, end_of_stream -> {}",
            context_id, body_size, end_of_stream
        );
        CallbackType(
            CallbackProto::ProxyOnRequestBody(context_id, body_size, end_of_stream),
            CallbackReturn::ReturnAction,
        )
    }

    fn call_proxy_on_request_trailers(&self, context_id: i32, num_trailers: i32) -> CallbackType {
        println!("CALL TO:   proxy_on_request_trailers");
        println!(
            "ARGS:      context_id -> {}, num_trailers -> {}",
            context_id, num_trailers
        );
        CallbackType(
            CallbackProto::ProxyOnRequestTrailers(context_id, num_trailers),
            CallbackReturn::ReturnAction,
        )
    }

    fn call_proxy_on_response_body(
        &self,
        context_id: i32,
        body_size: i32,
        end_of_stream: i32,
    ) -> CallbackType {
        println!("CALL TO:   proxy_on_response_body");
        println!(
            "ARGS:      context_id -> {}, body_size -> {}, end_of_stream -> {}",
            context_id, body_size, end_of_stream
        );
        CallbackType(
            CallbackProto::ProxyOnResponseBody(context_id, body_size, end_of_stream),
            CallbackReturn::ReturnAction,
        )
    }

    fn call_proxy_on_response_trailers(&self, context_id: i32, num_trailers: i32) -> CallbackType {
        println!("CALL TO:   proxy_on_response_trailers");
        println!(
            "ARGS:      context_id -> {}, num_trailers -> {}",
            context_id, num_trailers
        );
        CallbackType(
            CallbackProto::ProxyOnResponseTrailers(context_id, num_trailers),
            CallbackReturn::ReturnAction,
        )
    }

    fn call_proxy_on_http_call_response(
        &self,
        context_id: i32,
        callout_id: i32,
        num_headers: i32,
        body_size: i32,
        num_trailers: i32,
    ) -> CallbackType {
        println!("CALL TO:   proxy_on_http_call_response");
        println!(
            "ARGS:      context_id -> {}, callout_id -> {}",
            context_id, callout_id
        );
        println!(
            "           num_headers -> {}, body_size -> {}, num_trailers: {}",
            num_headers, body_size, num_trailers
        );
        CallbackType(
            CallbackProto::ProxyOnHttpCallResponse(
                context_id,
                callout_id,
                num_headers,
                body_size,
                num_trailers,
            ),
            CallbackReturn::ReturnEmpty,
        )
    }

    /* ---------------------------------- Combination Calls ---------------------------------- */
}

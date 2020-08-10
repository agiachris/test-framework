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

use anyhow::Result;
use proxy_wasm_test_framework::{tester, types::*};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let http_headers_path = &args[1];
    let mut http_headers_test = tester::test(http_headers_path)?;

    http_headers_test
        .call_start()
        .execute_and_expect(ReturnType::None)?;

    http_headers_test
        .call_proxy_on_context_create(1, 0)
        .execute_and_expect(ReturnType::None)?;

    http_headers_test
        .call_proxy_on_context_create(2, 1)
        .execute_and_expect(ReturnType::None)?;

    let header_map_pairs = vec![
        (":method", "GET"),
        (":path", "/hello"),
        (":authority", "developer"),
    ];
    let send_local_response_headers = vec![("Hello", "World"), ("Powered-By", "proxy-wasm")];
    http_headers_test
        .call_proxy_on_request_headers(2, 0)
        .expect_get_header_map_pairs(MapType::HttpRequestHeaders)
        .returning(header_map_pairs)
        .expect_get_header_map_value(MapType::HttpRequestHeaders, ":path")
        .returning("/hello")
        .expect_send_local_response(
            200,
            Some("Hello, World!\n"),
            send_local_response_headers,
            -1,
        )
        .execute_and_expect(ReturnType::Action(Action::Pause))?;

    let header_map_pairs = vec![
        (":method", "GET"),
        (":path", "/goodbye"),
        (":authority", "developer"),
    ];
    http_headers_test
        .call_proxy_on_response_headers(2, 0)
        .expect_get_header_map_pairs(MapType::HttpResponseHeaders)
        .returning(header_map_pairs)
        .expect_log(LogLevel::Trace, "#2 <- :method: GET")
        .expect_log(LogLevel::Trace, "#2 <- :path: /goodbye")
        .expect_log(LogLevel::Trace, "#2 <- :authority: developer")
        .execute_and_expect(ReturnType::Action(Action::Continue))?;

    http_headers_test
        .call_proxy_on_log(2)
        .expect_log(LogLevel::Trace, "#2 completed.")
        .execute_and_expect(ReturnType::None)?;

    return Ok(());
}

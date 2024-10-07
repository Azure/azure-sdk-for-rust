// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.

use cadl_specialwords::SpecialWordsClient;

#[async_std::test]
async fn with_and() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_and("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_as() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_as("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_assert() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_assert("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_async() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_async("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_await() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_await("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_break() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_break("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_cancellation_token() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_cancellation_token("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_class() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_class("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_constructor() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_constructor("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_continue() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_continue("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_def() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_def("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_del() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_del("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_elif() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_elif("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_else() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_else("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_except() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_except("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_exec() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_exec("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_finally() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_finally("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_for() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_for("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_from() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_from("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_global() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_global("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_if() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_if("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_import() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_import("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_in() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_in("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_is() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_is("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_lambda() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_lambda("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_not() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_not("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_or() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_or("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_pass() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_pass("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_raise() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_raise("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_return() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_return("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_try() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_try("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_while() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_while("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_with() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_with("ok", None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_yield() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_parameters_client()
        .with_yield("ok", None)
        .await
        .unwrap();
}

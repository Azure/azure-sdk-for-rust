// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.

use cadl_specialwords::SpecialWordsClient;

#[async_std::test]
async fn and() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .and(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn as_fn() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .as_fn(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn assert() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .assert(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn async_fn() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .async_fn(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn await_fn() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .await_fn(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn break_fn() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .break_fn(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn class() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .class(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn constructor() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .constructor(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn continue_fn() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .continue_fn(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn def() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .def(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn del() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .del(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn elif() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .elif(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn else_fn() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .else_fn(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn except() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .except(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn exec() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .exec(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn finally() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .finally(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn for_fn() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .for_fn(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn from() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .from(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn global() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .global(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn if_fn() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .if_fn(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn import() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .import(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn in_fn() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .in_fn(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn is() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .is(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn lambda() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .lambda(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn not() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .not(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn or() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .or(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn pass() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .pass(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn raise() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .raise(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn return_fn() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .return_fn(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn try_fn() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .try_fn(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn while_fn() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .while_fn(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .with(None)
        .await
        .unwrap();
}

#[async_std::test]
async fn yield_fn() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let _resp = client
        .get_special_words_operations_client()
        .yield_fn(None)
        .await
        .unwrap();
}

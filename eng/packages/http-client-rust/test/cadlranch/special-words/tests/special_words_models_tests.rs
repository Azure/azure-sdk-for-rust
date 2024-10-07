// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.

use cadl_specialwords::SpecialWordsClient;

use cadl_specialwords::models::{
    And, As, Assert, Async, Await, Break, Class, Constructor, Continue, Def, Del, Elif, Else,
    Except, Exec, Finally, For, From, Global, If, Import, In, Is, Lambda, Not, Or, Pass, Raise,
    Return, Try, While, With, Yield,
};

#[async_std::test]
async fn with_and() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = And::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_and(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_as() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = As::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_as(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_assert() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Assert::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_assert(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_async() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Async::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_async(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_await() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Await::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_await(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_break() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Break::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_break(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_class() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Class::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_class(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_constructor() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Constructor::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_constructor(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_continue() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Continue::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_continue(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_def() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Def::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_def(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_del() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Del::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_del(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_elif() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Elif::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_elif(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_else() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Else::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_else(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_except() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Except::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_except(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_exec() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Exec::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_exec(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_finally() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Finally::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_finally(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_for() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = For::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_for(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_from() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = From::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_from(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_global() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Global::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_global(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_if() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = If::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_if(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_import() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Import::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_import(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_in() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = In::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_in(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_is() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Is::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_is(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_lambda() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Lambda::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_lambda(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_not() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Not::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_not(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_or() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Or::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_or(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_pass() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Pass::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_pass(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_raise() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Raise::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_raise(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_return() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Return::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_return(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_try() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Try::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_try(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_while() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = While::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_while(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_with() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = With::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_with(req, None)
        .await
        .unwrap();
}

#[async_std::test]
async fn with_yield() {
    let client = SpecialWordsClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut body = Yield::default();
    body.name = Some(String::from("ok"));
    let req = body.try_into().unwrap();
    let _resp = client
        .get_special_words_models_client()
        .with_yield(req, None)
        .await
        .unwrap();
}

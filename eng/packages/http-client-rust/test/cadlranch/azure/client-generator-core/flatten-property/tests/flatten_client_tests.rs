// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.

use cadl_flattenproperty::models::{
    ChildFlattenModel, ChildModel, FlattenModel, NestedFlattenModel,
};
use cadl_flattenproperty::FlattenPropertyClient;

#[async_std::test]
async fn put_flatten_model() {
    let client = FlattenPropertyClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut child_model = ChildModel::default();
    child_model.age = Some(10);
    child_model.description = Some(String::from("bar"));
    let mut flatten_model = FlattenModel::default();
    flatten_model.name = Some(String::from("foo"));
    flatten_model.properties = Some(child_model);
    let req = flatten_model.try_into().unwrap();
    let resp = client.put_flatten_model(req, None).await.unwrap();
    let value: FlattenModel = resp.try_into().unwrap();
    assert_eq!(value.name, Some(String::from("test")));
    let props = value.properties.unwrap();
    assert_eq!(props.age, Some(1));
    assert_eq!(props.description, Some(String::from("test")));
}

#[async_std::test]
async fn put_nested_flatten_model() {
    let client = FlattenPropertyClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut child_model = ChildModel::default();
    child_model.age = Some(10);
    child_model.description = Some(String::from("test"));
    let mut child_flatten_model = ChildFlattenModel::default();
    child_flatten_model.properties = Some(child_model);
    child_flatten_model.summary = Some(String::from("bar"));
    let mut nested_flatten_model = NestedFlattenModel::default();
    nested_flatten_model.name = Some(String::from("foo"));
    nested_flatten_model.properties = Some(child_flatten_model);
    let req = nested_flatten_model.try_into().unwrap();
    let resp = client.put_nested_flatten_model(req, None).await.unwrap();
    let value: NestedFlattenModel = resp.try_into().unwrap();
    assert_eq!(value.name, Some(String::from("test")));
    let props = value.properties.unwrap();
    assert_eq!(props.summary, Some(String::from("test")));
    let props = props.properties.unwrap();
    assert_eq!(props.age, Some(1));
    assert_eq!(props.description, Some(String::from("foo")));
}

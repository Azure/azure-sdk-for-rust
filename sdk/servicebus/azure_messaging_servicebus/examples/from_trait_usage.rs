// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! This example demonstrates the usage of the From trait implementations for Message creation.

use azure_messaging_servicebus::Message;

fn main() {
    // Using From<&'static str>
    let message1: Message = "Hello from static str!".into();
    println!("Message 1 body: {}", message1.body_as_string().unwrap());

    // Using From<String>
    let text = String::from("Hello from owned String!");
    let message2: Message = text.into();
    println!("Message 2 body: {}", message2.body_as_string().unwrap());

    // Using From<&'static str> with explicit syntax
    let message3 = Message::from("Hello with explicit from!");
    println!("Message 3 body: {}", message3.body_as_string().unwrap());

    // Using From<String> with explicit syntax
    let dynamic_text = format!("Hello dynamic message {}", 42);
    let message4 = Message::from(dynamic_text);
    println!("Message 4 body: {}", message4.body_as_string().unwrap());

    println!("All From trait implementations work correctly!");
}

use agent::modules::connection::Connection;
use regex::Regex;
use speculoos::prelude::*;
mod helpers;
use helpers::run_test;

#[macro_use]
mod macros;

test! { smoke |cli| {
    let re = Regex::new(r"^agent-cli \d\.\d.\d").unwrap();
    let ret = agent!(cli, "--version");
    assert_that(&ret).matches(|v| re.is_match(v))
}}

test! { create_a_connection_and_list_connections |cli| {
    let connections = agent!(cli, "connection list");
    assert_that(&connections).is_equal_to(String::from("[]"));
    agent!(cli, "connection invite");
    let connections = agent!(cli, "connection list");
    let connections: Vec<Connection> = serde_json::from_str(&connections).unwrap();
    assert_that(&connections).has_length(1);
}}

test! { create_connection_and_send_a_message |cli| {
    agent!(cli, "connection invite");
    let connections_str = agent!(cli, "connection list");
    let connection: Vec<Connection> = serde_json::from_str(&connections_str).unwrap();
    let result = agent!(cli, "message --connection-id={} --message={}", connection[0].id, "bar");
    assert_that(&result).is_equal_to(String::from(""))
}}

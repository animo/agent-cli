use agent::modules::connection::Connection;
use helpers::run_test;
use speculoos::prelude::*;

mod helpers;

#[macro_use]
mod macros;

test! { create_a_connection_and_list_connections |cli| {
    let connections = agent!(cli, "connection list");
    assert_that(&connections).is_equal_to(String::from("[]"));
    agent!(cli, "connection invite");
    let connections_str = agent!(cli, "connection list");
    let connections: Vec<Connection> = serde_json::from_str(&connections_str).unwrap();
    assert_that(&connections).has_length(1);
}}

test! { create_connection_and_send_a_message |cli| {
    agent!(cli, "connection invite");
    let connections_str = agent!(cli, "connection list");
    let connections: Vec<Connection> = serde_json::from_str(&connections_str).unwrap();
    let result = agent!(cli, "message --connection-id={} --message={}", connections[0].connection_id, "bar");
    assert_that(&result).is_equal_to(String::from(""))
}}

test! { create_and_receive_an_invitation |cli| {
    let invite_url = agent!(cli, "connection invite");
    agent!(cli, "connection receive --url={}", invite_url);
    let connections_str = agent!(cli, "connection list");
    let connections: Vec<Connection> = serde_json::from_str(&connections_str).unwrap();
    assert_that(&connections).has_length(2);
}}

use std::time::Duration;

use angstrom_network::StromMessage;
use reth_provider::test_utils::NoopProvider;
use testing_tools::controllers::devnet::{AngstromDevnet, DevnetConfig};

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
#[serial_test::serial]
async fn test_broadcast_order_propagation() {
    reth_tracing::init_test_tracing();
    let config = DevnetConfig::default();
    let mut testnet = AngstromDevnet::spawn_devnet(NoopProvider::default(), config)
        .await
        .unwrap();

    // let orders = (0..3)
    //     .map(|_| generate_random_valid_order())
    //     .collect::<Vec<_>>();
    let orders = vec![];

    let delay_seconds = 4;
    let res = tokio::time::timeout(
        Duration::from_secs(delay_seconds),
        testnet.broadcast_orders_message(
            Some(0),
            StromMessage::PropagatePooledOrders(orders.clone()),
            orders.clone()
        )
    )
    .await
    .map(|r| r.unwrap());

    assert_eq!(
        res,
        Ok(true),
        "failed to receive and react to order within {} seconds",
        delay_seconds
    );

    let res = tokio::time::timeout(
        Duration::from_secs(delay_seconds),
        testnet.broadcast_orders_message(
            Some(0),
            StromMessage::PropagatePooledOrders(orders.clone()),
            orders
        )
    )
    .await
    .map(|r| r.unwrap());

    assert_eq!(res, Ok(true), "failed to receive and react to order within 4 seconds");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
#[serial_test::serial]
async fn test_singular_order_propagation() {
    reth_tracing::init_test_tracing();
    let config = DevnetConfig::default();

    // connect all peers
    //
    let testnet = tokio::time::timeout(
        Duration::from_secs(30),
        AngstromDevnet::spawn_devnet(NoopProvider::default(), config)
    )
    .await;
    assert!(matches!(testnet, Ok(Ok(_))), "failed to connect all peers within 30 seconds");

    let mut testnet = testnet.unwrap().unwrap();

    // let orders = (0..3)
    //     .map(|_| generate_random_valid_order())
    //     .collect::<Vec<_>>();
    let orders = vec![];

    let delay_seconds = 8;

    let res = tokio::time::timeout(
        Duration::from_secs(delay_seconds),
        testnet.broadcast_orders_message(
            Some(0),
            StromMessage::PropagatePooledOrders(orders.clone()),
            orders.clone()
        )
    )
    .await
    .map(|r| r.unwrap());

    assert_eq!(
        res,
        Ok(true),
        "failed to receive and react to order within {} seconds",
        delay_seconds
    );

    let res = tokio::time::timeout(
        Duration::from_secs(4),
        testnet.broadcast_orders_message(
            Some(0),
            StromMessage::PropagatePooledOrders(orders.clone()),
            orders.clone()
        )
    )
    .await
    .map(|r| r.unwrap());

    assert_eq!(res, Ok(true), "failed to receive and react to order within 4 seconds");
}

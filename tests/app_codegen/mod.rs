//! These tests should be compiled but not run.

#[tokio::test]
async fn test_basic_use() {
    let _app = celery::app!(
        broker = AMQPBroker { std::env::var("AMQP_ADDR").unwrap() },
        tasks = [],
        task_routes = []
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_basic_use_with_variable() {
    let connection_string = std::env::var("AMQP_ADDR").unwrap();
    let default_queue = "default";
    let _app = celery::app!(
        broker = AMQPBroker { connection_string },
        tasks = [],
        task_routes = [],
        default_queue = default_queue,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_basic_use_with_trailing_comma() {
    let _app = celery::app!(
        broker = AMQPBroker { std::env::var("AMQP_ADDR").unwrap() },
        tasks = [],
        task_routes = [],
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_with_options() {
    let _app = celery::app!(
        broker = AMQPBroker { std::env::var("AMQP_ADDR").unwrap() },
        tasks = [],
        task_routes = [],
        task_time_limit = 2
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_with_options_and_trailing_comma() {
    let _app = celery::app!(
        broker = AMQPBroker { std::env::var("AMQP_ADDR").unwrap() },
        tasks = [],
        task_routes = [],
        task_time_limit = 2,
    )
    .await
    .unwrap();
}

#[tokio::test]
fn test_with_empty_queues_keywords() {
    let _app = celery::app!(
        broker = AMQP { std::env::var("AMQP_ADDR").unwrap() },
        tasks = [],
        task_routes = [],
        queues=vec![]
    );
}

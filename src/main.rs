use tracing::instrument::WithSubscriber;
use tracing_subscriber::util::SubscriberInitExt;
use crate::task2022::codingtasks::task1::task_2022_task1;
use crate::task2022::codingtasks::task2::task_2022_task2;
use crate::task2022::codingtasks::task3::task_2022_task3;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
mod task2022;
mod util;


fn main() {
    println!("Hello, world!");
    //
    // let _ = tracing_subscriber::registry
    //     .with_subscriber(
    //         tracing_subscriber::EnvFilter::from_default_env()
    //             .add_directive(tracing_subscriber::filter::LevelFilter::DEBUG.into())
    //             ).init();


    let _ = tracing_subscriber::registry()
        .with(fmt::layer())
        // .with(EnvFilter::from_default_env())
        .with(EnvFilter::new("info"))
        .init();

    task_2022_task3();
}

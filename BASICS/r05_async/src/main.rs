
// use futures::prelude::*;
//use tokio::prelude::*;



// mod tokio_tcp_srv_mutex_gs;
// mod tokio_tcp_srv_atomic_gs;

mod gs_task;
mod tokio_tcp_srv_septask_gs;


async fn dummy_async_app() {
    println!("== Async Rust demo begin ==");

    // tokio_tcp_srv_mutex_gs::tcp_srv().await.unwrap();
    // tokio_tcp_srv_atomic_gs::tcp_srv().await.unwrap();
    tokio_tcp_srv_septask_gs::tcp_srv().await.unwrap();

    println!("== Async Rust demo end ==");
}


fn main() {
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = dummy_async_app();
    rt.block_on(future);
}


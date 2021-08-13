#[cfg(not(target_arch = "wasm32"))]
use std::thread;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;

use async_lock::Mutex;
use futures_lite::future;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn smoke() {
    future::block_on(async {
        let m = Mutex::new(());
        drop(m.lock().await);
        drop(m.lock().await);
    })
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn try_lock() {
    let m = Mutex::new(());
    *m.try_lock().unwrap() = ();
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn into_inner() {
    let m = Mutex::new(10i32);
    assert_eq!(m.into_inner(), 10);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn get_mut() {
    let mut m = Mutex::new(10i32);
    *m.get_mut() = 20;
    assert_eq!(m.into_inner(), 20);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn contention() {
    future::block_on(async {
        let (tx, rx) = async_channel::unbounded();

        let tx = Arc::new(tx);
        let mutex = Arc::new(Mutex::new(0i32));
        let num_tasks = 100;

        for _ in 0..num_tasks {
            let tx = tx.clone();
            let mutex = mutex.clone();

            thread::spawn(|| {
                future::block_on(async move {
                    let mut lock = mutex.lock().await;
                    *lock += 1;
                    tx.send(()).await.unwrap();
                    drop(lock);
                })
            });
        }

        for _ in 0..num_tasks {
            rx.recv().await.unwrap();
        }

        let lock = mutex.lock().await;
        assert_eq!(num_tasks, *lock);
    });
}

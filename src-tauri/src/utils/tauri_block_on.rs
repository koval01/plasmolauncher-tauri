use std::future::Future;

pub fn tauri_block_on<F>(future: F) -> F::Output
where F: Future
{
    tokio::task::block_in_place(|| { tauri::async_runtime::block_on(
        future
    )})
}
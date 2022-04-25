#![allow(dead_code, unused_imports, unused_variables)]

use candid::{CandidType, Principal};
use ic_cdk;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::{cell::RefCell, collections::BTreeMap, i64::MAX};

const WASM_PAGE_SIZE: usize = 65536;

#[derive(Debug, Default, CandidType, Deserialize, Clone)]
struct DataDog {
    data: BTreeMap<usize, Vec<u8>>,
}

#[derive(Debug, Default, CandidType, Deserialize, Clone)]
struct MemoryHeader {
    heap_pages: u64,
    heap_size: u64,
    stable_pages: u64,
    stable_size: u64,
    all: u64,
}

thread_local! {
    static STORE: RefCell<DataDog> = RefCell::new(DataDog::default());
}

#[update(name = "saveHeapToStableMemory")]
fn save_memory() {
    STORE.with(|m| {
        let m = &(*m.borrow_mut());

        // let tmp = DataDog::default();
        // let m = std::mem::replace(&mut m, &tmp);
        let m = m.clone();

        ic_cdk::storage::stable_save((m,))
            .expect("Fail to save the Canister state to stable memory.");
    });
}

#[update(name = "loadStableMemoryToHeap")]
fn load_stable_memory() {
    ic_cdk::println!(">> Loading memory back from stable...");

    let bytes = ic_cdk::api::stable::stable_bytes();

    let mut de = candid::de::IDLDeserialize::new(bytes.as_slice())
        .map_err(|e| format!("{:?}", e))
        .expect("nope 1");

    if let Ok((old_state,)) = candid::utils::ArgumentDecoder::decode(&mut de) {
        // bla(&old_state);
        STORE.with(|m| {
            let mut m = m.borrow_mut();
    
            *m = old_state
        });
    } else {
        ic_cdk::trap("Nope 2");
    }



    // match ic_cdk::storage::stable_restore::<(DataDog,)>() {
    //     Ok((old_state,)) => {
    //         STORE.with(|m| {
    //             let mut m = m.borrow_mut();

    //             *m = old_state
    //         });
    //     }
    //     Err(e) => {
    //         let emsg = &format!("Fail to restore Memory state from stable memory : {e:#?}");
    //         // ic_cdk::println!("{emsg}");
    //         ic_cdk::trap(emsg);
    //     }
    // };
}

#[update(name = "pushToHeapMemory")]
fn push_to_heap_memory(v: Vec<u8>) {
    STORE.with(|m| {
        let mut m = m.borrow_mut();

        let idx = m.data.keys().len();

        let v = std::borrow::Cow::from(&v);
        *m.data.entry(idx).or_insert(v.to_vec()) = v.to_vec();
        // *m.data.entry(idx).or_insert(v.clone()) = v.clone();
    });
}

#[query(name = "getCanisterMemoryHeader")]
fn get_canister_memory_header() -> MemoryHeader {
    let stable_pages = ic_cdk::api::stable::stable64_size();
    let stable_size = stable_pages * WASM_PAGE_SIZE as u64;

    let heap_pages = core::arch::wasm::memory_size(0) as u64;
    let heap_size = heap_pages * WASM_PAGE_SIZE as u64;

    MemoryHeader {
        heap_pages,
        heap_size,
        stable_pages,
        stable_size,
        all: heap_size + stable_size,
    }
}

#[update(name = "getTrustedCanisterMemoryHeader")]
fn get_trusted_canister_memory_header() -> MemoryHeader {
    let stable_pages = ic_cdk::api::stable::stable64_size();
    let stable_size = stable_pages * WASM_PAGE_SIZE as u64;

    let heap_pages = core::arch::wasm::memory_size(0) as u64;
    let heap_size = heap_pages * WASM_PAGE_SIZE as u64;

    MemoryHeader {
        heap_pages,
        heap_size,
        stable_pages,
        stable_size,
        all: heap_size + stable_size,
    }
}

#[query(name = "streamBackupStableMemory")]
fn stream_backup_stable_memory(offset: u64, size: usize) -> Vec<u8> {
    let mut buf = vec![0; WASM_PAGE_SIZE * size];
    ic_cdk::api::stable::stable64_read(offset, &mut buf);
    buf
}

#[update(name = "streamRestoreStableMemory")]
fn stream_restore_stable_memory(offset: u64, mut buf: Vec<u8>) {
    ic_cdk::api::stable::stable64_write(offset, &mut buf);
}

// #[pre_upgrade]
// fn pre_upgrade() {
//     STORE.with(|m| {
//         let m = &(*m.borrow_mut());

//         // let tmp = Memory::default();
//         // let m = std::mem::replace(&mut m, &tmp);

//         ic_cdk::storage::stable_save((&m,))
//             .expect("Fail to save the Canister state to stable memory.");
//     });
// }

// #[post_upgrade]
// fn post_upgrade() {
//     match ic_cdk::storage::stable_restore::<(DataDog,)>() {
//         Ok((old_state,)) => {
//             STORE.with(|m| {
//                 let mut m = m.borrow_mut();
//                 *m = old_state
//             });
//         }
//         Err(e) => {
//             let emsg = &format!("Fail to restore Memory state from stable memory : {e:#?}");
//             // ic_cdk::println!("{emsg}");
//             ic_cdk::trap(emsg);
//         }
//     };
// }

#[query(name = "getApiInfo")]
fn get_info_from_api() -> String {
    let mut s = String::new();

    s.push_str(&format!("\n"));
    s.push_str(&format!("Canister id = {}\n", ic_cdk::api::id()));
    s.push_str(&format!("Caller id = {}\n", ic_cdk::api::caller()));
    // s.push_str(&format!("msg_cycles_available = {}\n", ic_cdk::api::call::msg_cycles_available()));

    s
}

#[query(name = "greet")]
fn greet(name: String) -> String {
    format!("Hello my Lord {} !", name)
}

#[query(name = "greet2")]
fn greet3(name: String) -> String {
    format!("Hello my Lord {} !", name)
}

use candid::CandidType;
use ic_cdk_macros::*;
use serde::Deserialize;

#[pre_upgrade]
fn pre_upgrade() {
    let state: &State = ic_cdk::storage::get();
    ic_cdk::storage::stable_save((state,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (state,): (State,) = ic_cdk::storage::stable_restore().unwrap();
    *ic_cdk::storage::get_mut() = state;
}

#[query]
fn get_count() -> u32 {
    let state: &State = ic_cdk::storage::get();
    state.count
}

#[update]
fn count() -> u32 {
    let state: &mut State = ic_cdk::storage::get_mut();
    state.count += 1;
    state.count
}

#[update]
fn reset() {
    let state: &mut State = ic_cdk::storage::get_mut();
    std::mem::take(state);
}

#[derive(CandidType, Deserialize, Default)]
struct State {
    count: u32
}

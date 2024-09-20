use std::sync::atomic::Ordering;

pub fn increase_counter() {
    some_crate::GLOBAL_COUNTER.fetch_add(1, Ordering::AcqRel);
}

pub fn print_counter() {
    dbg!(&some_crate::GLOBAL_COUNTER);
}
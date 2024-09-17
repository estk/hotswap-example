use bindings::exports::hotswap::salutation::user_interface::{Guest, Time, User};

#[allow(warnings)]
mod bindings;

struct Component;

const DAY: u64 = 60 * 60 * 24;
const WEEK: u64 = DAY * 7;
const MONTH: u64 = DAY * 28;
const YEAR: u64 = WEEK * 52;

impl Guest for Component {
    fn age_in_weeks(_u: User, now: Time) -> u32 {
        let bd_sec: u64 = (1989 - 1970) * YEAR + 2 * MONTH;

        let since_bd_sec = now.seconds - bd_sec;
        (since_bd_sec / WEEK) as u32
    }
}

bindings::export!(Component with_types_in bindings);

use std::fmt::Display;

use bindings::{
    exports::hotswap::salutation::salutation::{FormalHonorific, Guest},
    hotswap::salutation::{
        user::age_in_weeks,
        user_types::{Gender, Time, User},
    },
};

#[expect(warnings)]
pub mod bindings;
struct Component;

impl Guest for Component {
    fn greet(u: User) -> String {
        let now = Time {
            seconds: 1_000_000_000,
            nanos: 0,
        };
        let weeks = age_in_weeks(&u, now);

        let fh = Component::get_formal_honorific(u);
        let weeks_left = 80 * 52 - weeks;
        format!("Greetings {fh}, you're {weeks} weeks old. Only {weeks_left} to go")
    }
    fn get_formal_honorific(u: User) -> FormalHonorific {
        match u.gender {
            Gender::Male => FormalHonorific::Sir,
            Gender::Female => FormalHonorific::Maam,
            Gender::Other => FormalHonorific::SirMaam,
        }
    }
}
impl Display for FormalHonorific {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sir => f.write_str("sir"),
            Self::Maam => f.write_str("ma'am"),
            Self::SirMaam => f.write_str("sir ma'am"),
            Self::Custom(c) => f.write_str(c),
        }
    }
}

bindings::export!(Component with_types_in bindings);

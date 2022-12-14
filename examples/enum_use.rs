#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

enum Epoll {
    Pending,
    Ready,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }

    use crate::Epoll::{Pending,Ready};

    let init_status = Pending;

    match init_status {
        Pending=>{
            println!("not ready yet");
        },
        Ready=>{
            println!("I'm ready");

        },
    }
}

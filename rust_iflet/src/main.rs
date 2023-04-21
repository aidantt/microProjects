// sometimes match is overkill for simple control flow.
// If control flow only needs to consider one value and throw away the rest,
// if let is sufficient and simpler.

fn main() {
    // let's say some code needs to be executed if a variable matches some value:
    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (), // unit tuple; do nothing for all other values
    // }
    // the _ => () catch-all is some annoying boilerplate to add.
    // This control flow can be executed cleaner using if let:
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // we still assign config_max as Option<u8>, and the control flow
    // still binds max to the data in Some(), but no catch-all is needed.

    // if let is a trade off: you lose the exhaustive case checking of match,
    // but it cleanly represents a singular control flow case with less boilerplate
    // and less syntax.
    // if let can also give an else {} case as a control flow for the catch-all in 
    // a binary case:
    let mut count = 0; // define count as i32 at 0 that can change value
}

use std::collections::VecDeque;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    representation: String,
}

struct TuringMachine {
    all_symbols: u8, // 0 is assumed to be the blank symbol
    all_states: u8, // 0 is assumed to be the start and
                    // the value of this is the halt
    transition_function: [[(u16, bool); 256]; 256],
    // the transition function maps from all possible
    // symbols and states to all symbols and states +
    // a tape move, L for false, R for true
}


fn main() {
    let args = Cli::from_args();
    let max_state = max_state(&args.representation);
    // 'a' is considered the initial state
    
    let max_sym = max_symbol(&args.representation);
    // 0 is considered the blank symbol

    println!("{}", max_state);
    println!("{}", max_sym);
    
    let mut tape: VecDeque<u8> = VecDeque::new();
    let mut pointer = 0;
}


fn max_state(rep: &str) -> u8 {
    // max state is z right now
    let mut max = 0;
    for c in rep.chars() {
	if !c.is_ascii_lowercase() {
	    continue;
	}
	
	let l = c as u8;
	if l - 97 > max {
	    max = l - 97;
	}
    }
    max
}

fn max_symbol(rep: &str) -> u8 {
    // max symbol is 9 right now
    let mut max = 0;
    for c in rep.chars() {
	if !c.is_ascii_digit() {
	    continue;
	}

	let l = c as u8;
	if l - 48 > max {
	    max = l - 48
	}
    }
    max
}

fn is_lexical_normal_form(rep: &str) -> bool {
    // lexical normal form:
    // *  first shift is R
    // *  the non-initial states occur in ascending order
    // *  the non-blank symbols occur in ascending order

    let mut last_symbol = 0;
    let mut seen_R = false;
    let mut last_state = 0;
    
    for c in rep.chars() {
	if c.is_ascii_digit() { // check symbols
	    let current_symbol = (c as u8) - 48;
	    if current_symbol > last_symbol + 1 {
		return false; // skipped a symbol
	    } else if current_symbol == last_symbol + 1 {
		last_symbol += 1;
	    }
	} else if c.is_ascii_uppercase() { // check shifts
	    if seen_R {
		continue;
	    } else if c == R {
		seen_R = true;
		continue;
	    } else if c == L {
		return false; // saw L first
	    } else {
		return false; // not a shift
	    }
	} else if c.is_ascii_lowercase() { // check states
	    let current_state = (c as u8) - 97;
	    if current_state > last_state + 1 {
		return false; // skipped a state
	    } else if current_state == last_state + 1 {
		last_state += 1;
	    }
	}
    }

    true
}

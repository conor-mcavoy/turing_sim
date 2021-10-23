use std::collections::VecDeque;
use structopt::StructOpt;

const SYMBOL_MAX: usize = 10;
const STATE_MAX: usize = 26;

#[derive(StructOpt)]
struct Cli {
    representation: String,
}

struct TuringMachine {
    all_symbols: u8, // 0 is assumed to be the blank symbol
    all_states: u8, // 0 is assumed to be the start and
                    // the value of this is the halt
    transition_function: [[(usize, bool, usize); SYMBOL_MAX]; STATE_MAX],
    // the transition function maps from all possible
    // symbols and states to all symbols and states +
    // a tape move, L for false, R for true
}


fn main() {
    let args = Cli::from_args();
    let max_state = max_state(&args.representation);
    // 'a' is considered the initial state
    // the max is the halt state
    
    let max_sym = max_symbol(&args.representation);
    // 0 is considered the blank symbol

    println!("{}", num_to_state(max_state));
    println!("{}", max_sym);

    if !is_lexical_normal_form(&args.representation) {
	panic!("Not in lexical normal form")
    }

    let tf: [[(u8, bool, u8); SYMBOL_MAX]; STATE_MAX] = populate_transition_function(&args.representation, max_state, max_sym);
    
    let mut tape: VecDeque<u8> = VecDeque::new();
    tape.push_back(0);
    let mut pointer: usize = 0;

    let mut curr_sym: u8 = 0;
    let mut curr_state: u8 = 0;

    for i in 0..200 {
	 for c in tape.iter() {
	     print!("{}", c);
	 }
	println!(": {}", num_to_state(curr_state));

	for x in 0..pointer {
	    print!(" ");
	}
	println!("^");
	
	let next_tuple: (u8, bool, u8) = tf[curr_sym as usize][curr_state as usize];

	let next_sym: u8 = next_tuple.0;
	let shift: bool = next_tuple.1;
	let next_state: u8 = next_tuple.2;

	//println!("pointer {}", pointer);
	tape[pointer] = next_sym;
	if shift {
	    pointer += 1;
	} else if pointer > 0 {
	    pointer -= 1;
	} else { // need to go negative, pointer is 0
	    tape.push_front(0);
	}

	if pointer >= tape.len() {
	    tape.push_back(0);
	}

	//println!("{}", tape.len());
	//println!("{}", pointer);
	curr_state = next_state;
	curr_sym = tape[pointer];

	if curr_state == max_state {
	    println!("halt");
	    break;
	}
    }
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

    let mut last_symbol: u8 = 0;
    let mut seen_r: bool = false;
    let mut last_state: u8 = 0;
    
    for c in rep.chars() {
	if c.is_ascii_digit() { // check symbols
	    let current_symbol = (c as u8) - 48;
	    if current_symbol > last_symbol + 1 {
		eprintln!("Skipped a symbol, not in LNF: {}", current_symbol);
		return false;
	    } else if current_symbol == last_symbol + 1 {
		last_symbol += 1;
	    }
	} else if c.is_ascii_uppercase() { // check shifts
	    if seen_r {
		continue;
	    } else if c == 'R' {
		seen_r = true;
		continue;
	    } else if c == 'L' {
		eprintln!("L shift first, not in LNF");
		return false;
	    } else {
		eprintln!("Not a valid shift, not in LNF: {}", c);
		return false;
	    }
	} else if c.is_ascii_lowercase() { // check states
	    let current_state = (c as u8) - 97;
	    if current_state > last_state + 1 {
		eprintln!("Skipped a state, not in LNF: {}", current_state);
		return false;
	    } else if current_state == last_state + 1 {
		last_state += 1;
	    }
	}
    }

    true
}

fn populate_transition_function(rep: &str, max_state: u8, max_sym: u8) -> [[(u8, bool, u8); SYMBOL_MAX]; STATE_MAX] {    
    let mut current_symbol: u8 = 0;
    let mut current_state: u8 = 0;
    let mut transition_function: [[(u8, bool, u8); SYMBOL_MAX]; STATE_MAX] = [[(0, false, 0); SYMBOL_MAX]; STATE_MAX];
    
    for rule in rep.split_ascii_whitespace() {
	// one rule is in the form symbol-shift-state

	// the below logic will have to be expanded if we ever want
	// to support > 10 symbols or > 26 states
	let rule_bytes: &[u8] = rule.as_bytes();
	let symbol_to_write: u8 = rule_bytes[0] - 48;
	let shift: bool = rule_bytes[1] == 82; // true for R
	let next_state: u8 = rule_bytes[2] - 97;

	println!("{}", rule);
	println!("Writing function at sym {} x state {} -> sym {} x shift {} x state {}", current_symbol, num_to_state(current_state), symbol_to_write, if shift {'R'} else {'L'}, num_to_state(next_state));
	
	transition_function[current_symbol as usize][current_state as usize] = (symbol_to_write, shift, next_state);

	// increment symbol first
	current_symbol += 1;
	// if it maxes out, wrap around
	if current_symbol == max_sym + 1 {
	    current_symbol = 0;
	    current_state += 1;
	}

	// if state maxes out, we should be done
	if current_state == max_state + 1 {
	    break;
	}
    }
    transition_function
}

fn state_to_num(state: char) -> u8 {
    state as u8 - 97
}

fn num_to_state(num: u8) -> char {
    (num + 97) as char
}

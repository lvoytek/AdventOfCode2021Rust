use std::fs;

#[derive(PartialEq)]
enum OpType {
    Var,
    Const,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
    Inp,
}

struct Operation {
    // The type of operation this is
    op_type: OpType,

    // The name of the variable / operation
    name: char,

    // The literal value of the node, used for Consts and when solving
    val: i64,

    // The model number digit this is associated with (Inp only)
    input_digit: usize,

    // The indecies of the child operations (Non Var and Const)
    arg_a_index: usize,
    arg_b_index: usize,

    // The current head node index for a variable's calculation (Var only)
    top: usize,
}

fn get_operation_index_of(arg: char, vars: &Vec<Operation>) -> usize {
    // Get the most recently added constant
    if arg == '-' {
        for i in (0..vars.len()).rev() {
            if vars[i].op_type == OpType::Const {
                return i;
            }
        }
    }

    // Get the associated variable's top index
    else {
        for i in 0..vars.len() {
            if vars[i].op_type == OpType::Var && vars[i].name == arg {
                return vars[i].top;
            }
        }
    }

    println!("Error, index of {} not found", arg);
    return 0;
}

fn set_top_to_latest(arg: char, vars: &mut Vec<Operation>) {
    for i in 0..vars.len() {
        if vars[i].op_type == OpType::Var && vars[i].name == arg {
            // Set new top to end of vector
            vars[i].top = vars.len();
            break;
        }
    }
}

fn alu_op(arg_a: char, arg_b: char, op_type: OpType, name: char, vars: &mut Vec<Operation>) {
    let idx_a = get_operation_index_of(arg_a, vars);
    let idx_b = get_operation_index_of(arg_b, vars);

    // Trim down operations if possible
    match op_type {
        OpType::Add => {
            // Nothing added, ignore this operation
            if vars[idx_b].op_type == OpType::Const && vars[idx_b].val == 0 {
                return;
            }

            // Item was 0, is now equal to the second val
            else if vars[idx_a].op_type == OpType::Const && vars[idx_a].val == 0 {
                for i in 0..vars.len() {
                    if vars[i].op_type == OpType::Var && vars[i].name == arg_a {
                        // Set new top to second index
                        vars[i].top = idx_b;
                        return;
                    }
                }
            }

            // Both values are constant, reduce
            else if vars[idx_a].op_type == OpType::Const && vars[idx_b].op_type == OpType::Const {
                set_top_to_latest(arg_a, vars);
                vars.push(Operation {
                    op_type: OpType::Const,
                    name: '-',
                    val: vars[idx_a].val + vars[idx_b].val,
                    input_digit: 0,
                    arg_a_index: 0,
                    arg_b_index: 0,
                    top: 0,
                });
                return;
            }
        },
        OpType::Mul => {
            // Multiplied by zero, set variable to constant zero
            if vars[idx_b].op_type == OpType::Const && vars[idx_b].val == 0 {
                for i in 0..vars.len() {
                    if vars[i].op_type == OpType::Var && vars[i].name == arg_a {
                        // Set new top to const 0 index
                        vars[i].top = idx_b;
                        return;
                    }
                }
            }
            else if vars[idx_a].op_type == OpType::Const && vars[idx_a].val == 0 {
                return;
            }

            // Multiplied by 1, ignore this operation
            else if vars[idx_b].op_type == OpType::Const && vars[idx_b].val == 1 {
                return;
            }

            // Item was 1, is now equal to the second val
            else if vars[idx_a].op_type == OpType::Const && vars[idx_a].val == 1 {
                for i in 0..vars.len() {
                    if vars[i].op_type == OpType::Var && vars[i].name == arg_a {
                        // Set new top to second index
                        vars[i].top = idx_b;
                        return;
                    }
                }
            }

            // Both values are constant, reduce
            else if vars[idx_a].op_type == OpType::Const && vars[idx_b].op_type == OpType::Const {
                set_top_to_latest(arg_a, vars);
                vars.push(Operation {
                    op_type: OpType::Const,
                    name: '-',
                    val: vars[idx_a].val * vars[idx_b].val,
                    input_digit: 0,
                    arg_a_index: 0,
                    arg_b_index: 0,
                    top: 0,
                });
                return;
            }
        },
        OpType::Div => {
            // Divided by 1, ignore this operation
            if vars[idx_b].op_type == OpType::Const && vars[idx_b].val == 1 {
                return;
            }

            // Both values are constant, reduce
            else if vars[idx_a].op_type == OpType::Const && vars[idx_b].op_type == OpType::Const {
                set_top_to_latest(arg_a, vars);
                vars.push(Operation {
                    op_type: OpType::Const,
                    name: '-',
                    val: vars[idx_a].val / vars[idx_b].val,
                    input_digit: 0,
                    arg_a_index: 0,
                    arg_b_index: 0,
                    top: 0,
                });
                return;
            }
        },
        OpType::Mod => {
            // Mod by 1, ignore this operation
            if vars[idx_b].op_type == OpType::Const && vars[idx_b].val == 1 {
                return;
            }

            // Both values are constant, reduce
            else if vars[idx_a].op_type == OpType::Const && vars[idx_b].op_type == OpType::Const {
                set_top_to_latest(arg_a, vars);
                vars.push(Operation {
                    op_type: OpType::Const,
                    name: '-',
                    val: vars[idx_a].val % vars[idx_b].val,
                    input_digit: 0,
                    arg_a_index: 0,
                    arg_b_index: 0,
                    top: 0,
                });
                return;
            }
        },
        OpType::Eql => {
            // Both values are constant, reduce
            if vars[idx_a].op_type == OpType::Const && vars[idx_b].op_type == OpType::Const {
                set_top_to_latest(arg_a, vars);
                vars.push(Operation {
                    op_type: OpType::Const,
                    name: '-',
                    val: if vars[idx_a].val == vars[idx_b].val {1} else {0},
                    input_digit: 0,
                    arg_a_index: 0,
                    arg_b_index: 0,
                    top: 0,
                });
                return;
            }

            // Digit input cannot by < 1 or > 9, therefore those consts compared with the digit result in 0
            else if vars[idx_a].op_type == OpType::Const && vars[idx_b].op_type == OpType::Inp && (vars[idx_a].val > 9 || vars[idx_a].val < 1) {
                set_top_to_latest(arg_a, vars);
                vars.push(Operation {
                    op_type: OpType::Const,
                    name: '-',
                    val: 0,
                    input_digit: 0,
                    arg_a_index: 0,
                    arg_b_index: 0,
                    top: 0,
                });
                return;
            }
            else if vars[idx_b].op_type == OpType::Const && vars[idx_a].op_type == OpType::Inp && (vars[idx_b].val > 9 || vars[idx_b].val < 1) {
                set_top_to_latest(arg_a, vars);
                vars.push(Operation {
                    op_type: OpType::Const,
                    name: '-',
                    val: 0,
                    input_digit: 0,
                    arg_a_index: 0,
                    arg_b_index: 0,
                    top: 0,
                });
                return;
            }
        },

        _ => (),
    }

    set_top_to_latest(arg_a, vars);

    vars.push(Operation {
        op_type: op_type,
        name: name,
        val: 0,
        input_digit: 0,
        arg_a_index: idx_a,
        arg_b_index: idx_b,
        top: 0,
    });
}

fn alu_add_op(arg_a: char, arg_b: char, vars: &mut Vec<Operation>) {
    alu_op(arg_a, arg_b, OpType::Add, '+', vars);
}

fn alu_mul_op(arg_a: char, arg_b: char, vars: &mut Vec<Operation>) {
    alu_op(arg_a, arg_b, OpType::Mul, '*', vars);
}

fn alu_div_op(arg_a: char, arg_b: char, vars: &mut Vec<Operation>) {
    alu_op(arg_a, arg_b, OpType::Div, '/', vars);
}

fn alu_mod_op(arg_a: char, arg_b: char, vars: &mut Vec<Operation>) {
    alu_op(arg_a, arg_b, OpType::Mod, '%', vars);
}

fn alu_eql_op(arg_a: char, arg_b: char, vars: &mut Vec<Operation>) {
    alu_op(arg_a, arg_b, OpType::Eql, '=', vars);
}

fn alu_inp_op(arg_a: char, digit_num: usize, vars: &mut Vec<Operation>) {
    set_top_to_latest(arg_a, vars);

    vars.push(Operation {
        op_type: OpType::Inp,
        name: ':',
        val: 0,
        input_digit: digit_num,
        arg_a_index: 0,
        arg_b_index: 0,
        top: 0,
    });
}


// Return the next digit num if inp otherwise current digit num
fn alu_inst(instruction: &str, digit_num: usize, vars: &mut Vec<Operation>) -> usize {
    let args = instruction.split_whitespace().collect::<Vec<&str>>();
    let arg_a = args[1].chars().nth(0).unwrap();

    let mut arg_b = '-';

    if args.len() > 2 {
        arg_b = args[2].chars().nth(0).unwrap();

        // 2nd arg is an immediate value, not a variable, create a temp var to reflect this
        if arg_b < 'a' || arg_b > 'z' {
            arg_b = '-';

            vars.push(Operation {
                op_type: OpType::Const,
                name: '-',
                val: args[2].parse::<i64>().unwrap(),
                input_digit: 0,
                arg_a_index: 0,
                arg_b_index: 0,
                top: 0,
            });
        }
    }

    match args[0] {
        "add" => alu_add_op(arg_a, arg_b, vars),
        "mul" => alu_mul_op(arg_a, arg_b, vars),
        "div" => alu_div_op(arg_a, arg_b, vars),
        "mod" => alu_mod_op(arg_a, arg_b, vars),
        "eql" => alu_eql_op(arg_a, arg_b, vars),
        "inp" => {
            alu_inp_op(arg_a, digit_num, vars);
            return digit_num + 1;
        }
        _ => println!("Error, invalid command"),
    }

    return digit_num;
}

fn alu_print(index: usize, vars: &Vec<Operation>) {
    match vars[index].op_type {
        OpType::Const => print!("{}", vars[index].val),
        OpType::Var => print!("{}", vars[index].name),
        OpType::Inp => {
            print!("d{}", vars[index].input_digit);
        },
        _ => {
            print!("(");
            alu_print(vars[index].arg_a_index, vars);
            print!("{}", vars[index].name);
            alu_print(vars[index].arg_b_index, vars);
            print!(")");
        },
    }
}

fn alu_rev_solve(results: &Vec<i64>, index: usize, digits: &mut [u8; 14], vars: &Vec<Operation>) -> i64 {
    match vars[index].op_type {
        OpType::Var | OpType::Const => {return vars[index].val},
        OpType::Add => {

            let rhs = alu_solve(vars[index].arg_b_index, digits, vars);
            let mut lhs_results = Vec::<i64>::new();
            for i in 0..results.len() {
                lhs_results.push(results[i] - rhs);
            }

            return alu_rev_solve(&lhs_results, vars[index].arg_a_index, digits, vars) + rhs;
        },
        OpType::Mul => {

            let rhs = alu_solve(vars[index].arg_b_index, digits, vars);
            let mut lhs_results = Vec::<i64>::new();
            for i in 0..results.len() {
                lhs_results.push(results[i] / rhs);
            }

            return alu_rev_solve(&lhs_results, vars[index].arg_a_index, digits, vars) / rhs;
        },
        OpType::Div => return alu_solve(vars[index].arg_a_index, digits, vars) / alu_solve(vars[index].arg_b_index, digits, vars),
        OpType::Mod => return alu_solve(vars[index].arg_a_index, digits, vars) % alu_solve(vars[index].arg_b_index, digits, vars),
        OpType::Eql => return if alu_solve(vars[index].arg_a_index, digits, vars) == alu_solve(vars[index].arg_b_index, digits, vars) {1} else {0},
        OpType::Inp => {
            // Find max result within range [1,9]
            let mut max_result = 0;
            for i in 0..results.len() {
                if results[i] < 10 && results[i] > max_result {
                    max_result = results[i];
                }
            }

            if max_result < 1 {
                println!("Error: no digit result found for d{}", vars[index].input_digit);
            }

            digits[vars[index].input_digit] = max_result as u8;

            return max_result;
        },
    }
}

fn alu_solve(index: usize, digits: &[u8; 14], vars: &Vec<Operation>) -> i64 {
    match vars[index].op_type {
        OpType::Var | OpType::Const => {return vars[index].val},
        OpType::Add => return alu_solve(vars[index].arg_a_index, digits, vars) + alu_solve(vars[index].arg_b_index, digits, vars),
        OpType::Mul => return alu_solve(vars[index].arg_a_index, digits, vars) * alu_solve(vars[index].arg_b_index, digits, vars),
        OpType::Div => return alu_solve(vars[index].arg_a_index, digits, vars) / alu_solve(vars[index].arg_b_index, digits, vars),
        OpType::Mod => return alu_solve(vars[index].arg_a_index, digits, vars) % alu_solve(vars[index].arg_b_index, digits, vars),
        OpType::Eql => return if alu_solve(vars[index].arg_a_index, digits, vars) == alu_solve(vars[index].arg_b_index, digits, vars) {1} else {0},
        OpType::Inp => return digits[vars[index].input_digit] as i64,
    }
}

fn main() {
    let input_contents = fs::read_to_string("files/small_monad_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();
    let mut vars = Vec::<Operation>::new();

    // Push number vars onto list
    vars.push(Operation {
        op_type: OpType::Var,
        name: 'w',
        val: 0,
        input_digit: 0,
        arg_a_index: 0,
        arg_b_index: 0,
        top: 4,
    });

    vars.push(Operation {
        op_type: OpType::Var,
        name: 'x',
        val: 0,
        input_digit: 0,
        arg_a_index: 0,
        arg_b_index: 0,
        top: 4,
    });

    vars.push(Operation {
        op_type: OpType::Var,
        name: 'y',
        val: 0,
        input_digit: 0,
        arg_a_index: 0,
        arg_b_index: 0,
        top: 4,
    });

    vars.push(Operation {
        op_type: OpType::Var,
        name: 'z',
        val: 0,
        input_digit: 0,
        arg_a_index: 0,
        arg_b_index: 0,
        top: 4,
    });

    // Push initial state of 0 for all vars
    vars.push(Operation {
        op_type: OpType::Const,
        name: '-',
        val: 0,
        input_digit: 0,
        arg_a_index: 0,
        arg_b_index: 0,
        top: 0,
    });

    let mut current_digit = 0;

    // Build a tree of operations for testing digits
    for line in &lines {
        current_digit = alu_inst(line, current_digit, &mut vars);
    }

    let mut digits : [u8; 14] = [9; 14];

    let possible_results: Vec<i64> = vec![0; 1];

    alu_print(vars[3].top, &vars);
    alu_rev_solve(&possible_results, vars[3].top, &mut digits, &vars);
    assert_eq!(0, alu_solve(vars[3].top, &digits, &vars));

    for i in 0..14 {
        print!("{}", digits[i]);
    }

    println!("");
}

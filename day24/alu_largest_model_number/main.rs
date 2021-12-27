use std::fs;
use std::cmp::Ordering;

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

#[derive(Clone, Copy, Eq)]
struct SolutionDigit {
    used: bool,
    value: u8,
}

impl PartialOrd for SolutionDigit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SolutionDigit {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.used && other.used {
            return self.value.cmp(&other.value);
        }
        else if !self.used && !other.used {
            return Ordering::Equal;
        }
        else if self.used {
            return Ordering::Less;
        }
        else {
            return Ordering::Greater;
        }
    }
}

impl PartialEq for SolutionDigit {
    fn eq(&self, other: &Self) -> bool {
        self.used && other.used && self.value == other.value
    }
}

#[derive(Clone, Copy, Eq)]
struct Solution {
    result: i64,
    digits: [SolutionDigit; 14],
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Solution {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.result == other.result {
            for i in 0..14 {
                if self.digits[i] != other.digits[i] {
                    if self.digits[i].used && other.digits[i].used {
                        return self.digits[i].cmp(&other.digits[i]);
                    }
                    else if self.digits[i].used || other.digits[i].used {
                        return Ordering::Equal;
                    }
                }
            }

            return Ordering::Equal;
        }
        else {
            return self.result.cmp(&other.result);
        }
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        if self.result == other.result {
            for i in 0..14 {
                if self.digits[i] != other.digits[i] {
                    return false;
                }
            }
            return true;
        }
        else {
            return false;
        }
    }
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

#[allow(dead_code)]
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

fn create_empty_solution(result: i64) -> Solution {
    return Solution {
        result: result,
        digits: [SolutionDigit {
                used: false,
                value: 0,
        }; 14],
    };
}

// If combination of two solutions succeeds return it and true otherwise false
fn combine_solutions(result: i64, solution_a: &Solution, solution_b: &Solution) -> (Solution, bool) {
    let mut new_solution = create_empty_solution(result);

    for i in 0..14 {
        if solution_a.digits[i].used && solution_b.digits[i].used {
            if solution_a.digits[i].value == solution_b.digits[i].value {
                new_solution.digits[i].used = true;
                new_solution.digits[i].value = solution_a.digits[i].value;
            }
            else {
                return (new_solution, false);
            }
        }
        else if solution_a.digits[i].used {
            new_solution.digits[i].used = true;
            new_solution.digits[i].value = solution_a.digits[i].value;
        }
        else if solution_b.digits[i].used {
            new_solution.digits[i].used = true;
            new_solution.digits[i].value = solution_b.digits[i].value;
        }
    }

    return (new_solution, true);
}

// Keep top solutions per output value
fn trim_solutions(solutions: &mut Vec<Solution>) {
    for i in (0..solutions.len()).rev() {
        let mut add_solution = true;

        for j in 0..solutions.len() {
            if j != i && solutions[j].result == solutions[i].result && solutions[i] < solutions[j] {
                add_solution = false;
                break;
            }
        }

        if !add_solution {
            solutions.remove(i);
        }
    }
}

fn alu_any_solve(index: usize, vars: &Vec<Operation>) -> Vec<Solution> {
    match vars[index].op_type {
        OpType::Var | OpType::Const => {
            return vec![create_empty_solution(vars[index].val); 1];
        },
        OpType::Add => {
            let rhs_solutions = alu_any_solve(vars[index].arg_b_index, vars);
            let lhs_solutions = alu_any_solve(vars[index].arg_a_index, vars);
            let mut full_solutions = Vec::<Solution>::new();

            for rhs_solution in &rhs_solutions {
                for lhs_solution in &lhs_solutions {
                    let (new_solution, success) = combine_solutions(lhs_solution.result + rhs_solution.result, lhs_solution, rhs_solution);

                    if success {
                        full_solutions.push(new_solution);
                    }
                }
            }

            trim_solutions(&mut full_solutions);
            return full_solutions;
        }
        OpType::Mul => {
            let rhs_solutions = alu_any_solve(vars[index].arg_b_index, vars);
            let lhs_solutions = alu_any_solve(vars[index].arg_a_index, vars);
            let mut full_solutions = Vec::<Solution>::new();

            for rhs_solution in &rhs_solutions {
                for lhs_solution in &lhs_solutions {
                    let (new_solution, success) = combine_solutions(lhs_solution.result * rhs_solution.result, lhs_solution, rhs_solution);

                    if success {
                        full_solutions.push(new_solution);
                    }
                }
            }

            trim_solutions(&mut full_solutions);
            return full_solutions;
        },
        OpType::Div => {
            let rhs_solutions = alu_any_solve(vars[index].arg_b_index, vars);
            let lhs_solutions = alu_any_solve(vars[index].arg_a_index, vars);
            let mut full_solutions = Vec::<Solution>::new();

            for rhs_solution in &rhs_solutions {
                for lhs_solution in &lhs_solutions {
                    let (new_solution, success) = combine_solutions(lhs_solution.result / rhs_solution.result, lhs_solution, rhs_solution);

                    if success {
                        full_solutions.push(new_solution);
                    }
                }
            }

            trim_solutions(&mut full_solutions);
            return full_solutions;
        },
        OpType::Mod => {
            let rhs_solutions = alu_any_solve(vars[index].arg_b_index, vars);
            let lhs_solutions = alu_any_solve(vars[index].arg_a_index, vars);
            let mut full_solutions = Vec::<Solution>::new();

            for rhs_solution in &rhs_solutions {
                for lhs_solution in &lhs_solutions {
                    let (new_solution, success) = combine_solutions(lhs_solution.result % rhs_solution.result, lhs_solution, rhs_solution);

                    if success {
                        full_solutions.push(new_solution);
                    }
                }
            }

            trim_solutions(&mut full_solutions);
            return full_solutions;
        },
        OpType::Eql => {
            let rhs_solutions = alu_any_solve(vars[index].arg_b_index, vars);
            let lhs_solutions = alu_any_solve(vars[index].arg_a_index, vars);
            let mut full_solutions = Vec::<Solution>::new();

            for rhs_solution in &rhs_solutions {
                for lhs_solution in &lhs_solutions {
                    let (new_solution, success) = combine_solutions(if lhs_solution.result == rhs_solution.result {1} else {0}, lhs_solution, rhs_solution);

                    if success {
                        full_solutions.push(new_solution);
                    }
                }
            }

            trim_solutions(&mut full_solutions);
            return full_solutions;
        },
        OpType::Inp => {
            let mut solution_vec = Vec::<Solution>::new();

            // Use all digit vals 1-9
            for i in 1..=9 {
                let mut new_solution = create_empty_solution(i);
                new_solution.digits[vars[index].input_digit].used = true;
                new_solution.digits[vars[index].input_digit].value = i as u8;
                solution_vec.push(new_solution);
            }

            return solution_vec;
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
    let input_contents = fs::read_to_string("files/monad_input")
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

    //alu_print(vars[3].top, &vars);
    let solutions = alu_any_solve(vars[3].top, &vars);
    println!("{}", solutions.len());

    for i in 0..solutions.len() {
        print!("{} ", solutions[i].result);
    }
    println!("");

    // Find zero solution
    let mut largest_zero_index: usize = 0;

    for i in 0..solutions.len() {
        if solutions[i].result == 0 {
            largest_zero_index = i;
            break;
        }
    }

    // Extract digits
    let mut digits : [u8; 14] = [9; 14];

    for i in 0..14 {
        if solutions[largest_zero_index].digits[i].used {
            digits[i] = solutions[largest_zero_index].digits[i].value;
        }
    }

    assert_eq!(0, alu_solve(vars[3].top, &digits, &vars));

    for i in 0..14 {
        print!("{}", digits[i]);
    }

    println!("");
}

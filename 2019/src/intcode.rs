use std::{error::Error, fmt, panic};

#[derive(Debug, Clone)]
struct SegmentationFault;

impl fmt::Display for SegmentationFault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Segmentation fault")
    }
}

impl Error for SegmentationFault {}

pub fn run_intcode(
    initial_mem: &[usize],
    input1: usize,
    input2: usize,
) -> Result<usize, Box<dyn Error>> {
    let mut instruction_pointer = 0;
    let mut mem = initial_mem.to_owned();
    mem[1] = input1;
    mem[2] = input2;

    loop {
        match step(&mut mem, instruction_pointer)? {
            StepResult::Halt => return Ok(mem.get(0).expect("There should be a 0 memory").clone()),
            StepResult::Continue(next_instruction_pointer) => {
                instruction_pointer = next_instruction_pointer
            }
        };
    }
}

fn get_pointer_argument(
    mem: &mut [usize],
    instruction_pointer: usize,
    argument_index: usize,
) -> Result<usize, Box<dyn Error>> {
    let argument_pointer = get_argument(mem, instruction_pointer, argument_index)?;
    get_mem(mem, argument_pointer)
}

fn get_argument(
    mem: &mut [usize],
    instruction_pointer: usize,
    argument_index: usize,
) -> Result<usize, Box<dyn Error>> {
    get_mem(mem, instruction_pointer + 1 + argument_index)
}

fn get_mem(mem: &mut [usize], pointer: usize) -> Result<usize, Box<dyn Error>> {
    let value = mem.get(pointer).ok_or(Box::new(SegmentationFault))?;
    Ok(value.clone())
}

fn set_mem(mem: &mut [usize], pointer: usize, value: usize) -> Result<(), Box<dyn Error>> {
    let mut_pos = mem.get_mut(pointer).ok_or(Box::new(SegmentationFault))?;
    *mut_pos = value;
    Ok(())
}

fn operation_sum(mem: &mut [usize], instruction_pointer: usize) -> Result<usize, Box<dyn Error>> {
    let value = get_pointer_argument(mem, instruction_pointer, 0)?
        + get_pointer_argument(mem, instruction_pointer, 1)?;

    let argument_pointer = get_argument(mem, instruction_pointer, 2)?;
    set_mem(mem, argument_pointer, value)?;

    Ok(instruction_pointer + 4)
}

fn operation_product(
    mem: &mut [usize],
    instruction_pointer: usize,
) -> Result<usize, Box<dyn Error>> {
    let value = get_pointer_argument(mem, instruction_pointer, 0)?
        * get_pointer_argument(mem, instruction_pointer, 1)?;

    let argument_pointer = get_argument(mem, instruction_pointer, 2)?;
    set_mem(mem, argument_pointer, value)?;

    Ok(instruction_pointer + 4)
}

enum StepResult {
    Halt,
    Continue(usize),
}

fn step(mem: &mut Vec<usize>, instruction_pointer: usize) -> Result<StepResult, Box<dyn Error>> {
    match get_mem(mem, instruction_pointer)? {
        1 => Ok(StepResult::Continue(operation_sum(
            mem,
            instruction_pointer,
        )?)),
        2 => Ok(StepResult::Continue(operation_product(
            mem,
            instruction_pointer,
        )?)),
        99 => Ok(StepResult::Halt),
        _ => Ok(StepResult::Halt),
    }
}

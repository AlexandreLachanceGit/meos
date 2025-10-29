const MAX_PROCESS: usize = 8;
const STACK_SIZE: usize = 8192;

struct Process {
    pid: usize,
    stack_pointer: usize,
    stack: [usize; STACK_SIZE],
}

#[derive(Default)]
pub struct ProcessManager {
    procs: [Option<Process>; MAX_PROCESS],
}

impl ProcessManager {
    fn new_process(&mut self, program_counter: usize) -> Result<usize, &'static str> {
        let (id, proc) = self
            .procs
            .iter_mut()
            .enumerate()
            .find(|(_, p)| p.is_none())
            .ok_or("Max processes reached")?;

        let mut process = Process {
            pid: id,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
        };

        process.stack[STACK_SIZE - 1] = 0; // s11
        process.stack[STACK_SIZE - 2] = 0; // s10
        process.stack[STACK_SIZE - 3] = 0; // s9
        process.stack[STACK_SIZE - 4] = 0; // s8
        process.stack[STACK_SIZE - 5] = 0; // s7
        process.stack[STACK_SIZE - 6] = 0; // s6
        process.stack[STACK_SIZE - 7] = 0; // s5
        process.stack[STACK_SIZE - 8] = 0; // s4
        process.stack[STACK_SIZE - 9] = 0; // s3
        process.stack[STACK_SIZE - 10] = 0; // s2
        process.stack[STACK_SIZE - 11] = 0; // s1
        process.stack[STACK_SIZE - 12] = 0; // s0
        process.stack[STACK_SIZE - 13] = program_counter; // ra

        *proc = Some(process);
        Ok(id)
    }
}

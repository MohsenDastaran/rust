#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: i32,
    hard_drive: i32,
}

impl Computer {
    fn new(cpu: String, memory: i32, hard_drive: i32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive,
        }
    }
    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }
    fn upgrade_memory(&mut self, new: i32) -> &mut Self{
        self.memory = new;
        self
    }
    fn upgrade_hard_drive(&mut self, new_space: i32)-> &mut Self {
        self.hard_drive = new_space;
        self
    }
}
fn main() {
    let mut my_computer = Computer::new("Intel".to_string(), 16, 512);


    println!("Hello, {:?}", my_computer);

    my_computer.upgrade_cpu("AMD".to_string()).upgrade_hard_drive(120).upgrade_memory(16);

    println!("New, {:?}", my_computer);

}

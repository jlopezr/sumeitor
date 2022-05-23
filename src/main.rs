use std::time::Instant;
use rand::Rng;

#[macro_use]
extern crate text_io;

//TODO Is better to implement Copy? Or to pass a reference?
#[derive(Clone, Copy)]
enum OpType {
    Add,
    Sub,
}

fn to_str(op: OpType) -> &'static str {
    match op {
        OpType::Add => "+",
        OpType::Sub => "-",
    }
}

fn do_op(op: OpType, a: i32, b: i32) -> i32 {
    match op {
        OpType::Add => a+b,
        OpType::Sub => a-b,
    }
}

fn main() {

    let mut good = 0;
    let mut bad = 0;
    let mut input: i32 = 69;
    let start = Instant::now();
    let mut rng = rand::thread_rng();
    let mut end = false;
    
    while !end {
    
        let a = rng.gen_range(0..99);
        let b = rng.gen_range(0..10);
        let op: OpType = OpType::Add;

        println!();
        println!("  {:>2}", a);
        println!("{} {:>2}", to_str(op), b);
        println!("------");
        
        input = read!();
        
        if input == do_op(op, a, b) {
            good = good+1;
        } else {
            bad = bad+1;
        }

        if start.elapsed().as_secs() > 60*1 {
            end = true;
            println!();
            println!("Respuestas:");
            println!("Totales:   {}", good+bad);
            println!("Correctas: {}", good);

            //TODO Guardar log en archivo
        }
    }

}

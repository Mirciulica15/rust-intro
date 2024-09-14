use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

lazy_static!(
    static ref GLOBAL_MAP: Mutex<HashMap<i64, i64>> = Mutex::new(HashMap::new());
);

fn main() {
    let x: i32 = 10;
    for i in 0..x {
        println!("This is number {}", i);
    }

    let a: i32 = 4;
    let b: i32 = 6;

    println!("{}", test_function(a, b));
    println!("This is a: {}, this is b: {}", a, b);

    let my_string = String::from("HELLO");
    let retake_string = test_string_function(&my_string);
    println!("Test string function {}", retake_string);
    println!("Original string here {}", my_string);

    let n: i64 = 30;

    let start_time_normal_fibonacci = Instant::now();
    println!("The {}th number in the Fibonacci sequence is {}", n + 1, normal_fibonacci(n));
    let duration_normal_fibonacci = start_time_normal_fibonacci.elapsed();
    println!("Execution took {:?}", duration_normal_fibonacci);

    let start_time = Instant::now();
    println!("The {}th number in the Fibonacci sequence is {}", n + 1, test_recursive_function(n));
    let duration = start_time.elapsed();
    println!("Execution took {:?}", duration);
}

// Primitive types are stored on stack, no borrowing required
fn test_function(a: i32, b: i32) -> i32 {
    let c = a + b;
    c
}

// String is stored on heap, must be borrowed or referenced
fn test_string_function(a: &String) -> &String {
    &a
}

fn normal_fibonacci(a: i64) -> i64 {
    if a == 0 || a == 1 {
        return 1;
    }
    normal_fibonacci(a - 1) + normal_fibonacci(a - 2)
}

fn test_recursive_function(a: i64) -> i64 {
    {
        let mut map = GLOBAL_MAP.lock().unwrap();
        if a == 0 || a == 1 {
            map.insert(a, 1);
            return 1;
        }
        if let Some(b) = map.get(&a).cloned() {
            return b;
        }
    }
    let result: i64 = test_recursive_function(a - 1) + test_recursive_function(a - 2);
    {
        let mut map = GLOBAL_MAP.lock().unwrap();
        map.insert(a, result);
    }
    result
}

use std::thread;

#[derive(Clone, Copy)]
struct MySafePointer {
    p: *mut i32
}

unsafe impl Send for MySafePointer {}

fn main() {
    let mut i = 0;
    let pi = MySafePointer { p: &mut i as *mut i32 };

    let h1 = thread::spawn(move || {
        let pi = pi;
        for _ in 0..10000 {
            unsafe { *pi.p += 1; }
        }
    });

    let h2 = thread::spawn(move || {
        let pi = pi;
        for _ in 0..10000 {
            unsafe { *pi.p += 1; }
        }
    });

    h1.join().unwrap();
    h2.join().unwrap();

    println!("{}", unsafe { *pi.p });
}

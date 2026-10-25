fn main() {
    //println!("Hello, world!");
    let a = String::from("sample buffer");
    let _b = a;
    //println!("{}", a);

	let s = String::from("hi");
	println!("{:p}", s.as_ptr());   // address of the heap bytes
	let t = s;                      // move
	println!("{:p}", t.as_ptr());   // same address: only the 24-byte handle moved
	println!("{}", std::mem::size_of::<String>());  // 24: pointer + length + capacity
}

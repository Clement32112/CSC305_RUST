//pub const vector:vec = vec!(1,2,3); ;
pub mod tuple {

    pub fn run() {
        let t1 = (12, "Hello", true, Some(12));
        println!("tuple{:?}", t1);
    }
}

pub mod array {
    pub fn run() {
        let a1: [i8; 5] = [1, 2, 3, 4, 5];
        println!("Array: {:?}", a1);
    }
}

pub mod slice {
    pub fn run() {
        let a1: [i8; 5] = [1, 2, 3, 4, 5];
        let a2 = "abcdefg";
        print!("Array2: {:?}", a1);
        println!("\tArray Slice: {:?}", a1.split_at(3).0);
        print!("String, {}", a2);
        println!("\tString Slice, {}", a2.split_at(3).0);
    }
}

pub mod vectors {

    pub fn run() {
        let v1 = vec![1, 2, 34];
        println!("vec: {:?}", v1);
    }
}

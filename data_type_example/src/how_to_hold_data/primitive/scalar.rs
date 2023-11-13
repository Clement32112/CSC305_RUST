#[allow(dead_code)]
pub mod integer {
    
    pub fn run() {
        let max_integer8: i8 = i8::MAX;
        let min_integer8: i8 = i8::MIN;

        let max_integer16: i16 = i16::MAX;
        let min_integer16: i16 = i16::MIN;

        let max_integer32: i32 = i32::MAX;
        let min_integer32: i32 = i32::MIN;

        let max_integer64: i64 = i64::MAX;
        let min_integer64: i64 = i64::MIN;

        println!(
            "Integer of 8 bit,\tmin:{}\t\t\tmax:{} ",
            min_integer8, max_integer8
        );
        println!(
            "Integer of 16 bit,\tmin:{}\t\t\tmax:{} ",
            min_integer16, max_integer16
        );
        println!(
            "Integer of 32 bit,\tmin:{}\t\t\tmax:{} ",
            min_integer32, max_integer32
        );
        println!(
            "Integer of 64 bit,\tmin:{}\tmax:{} ",
            min_integer64, max_integer64
        );
    }
}

pub mod unsigned_integer {

    pub fn run() {
        let min_u_integer8: u8 = u8::MIN;
        let max_u_integer8: u8 = u8::MAX;

        let min_u_integer16: u16 = u16::MIN;
        let max_u_integer16: u16 = u16::MAX;

        let min_u_integer32: u32 = u32::MIN;
        let max_u_integer32: u32 = u32::MAX;

        let min_u_integer64: u64 = u64::MIN;
        let max_u_integer64: u64 = u64::MAX;

        println!(
            "u_Integer of 8 bit,\tmin:{}\t\t\tmax:{} ",
            min_u_integer8, max_u_integer8
        );
        println!(
            "u_Integer of 16 bit,\tmin:{}\t\t\tmax:{} ",
            min_u_integer16, max_u_integer16
        );
        println!(
            "u_Integer of 32 bit,\tmin:{}\t\t\tmax:{} ",
            min_u_integer32, max_u_integer32
        );
        println!(
            "u_Integer of 64 bit,\tmin:{}\tmax:{} ",
            min_u_integer64, max_u_integer64
        );
    }
}

pub mod float {

    pub fn run() {
        let float32: f32 = 0.1232348349458234903984723973485235745389453845;
        let float64: f64 = 0.1232348349458234903984723973485235745389453845;

        println!("float of 32 bit: \t{} ", float32);
        println!("float of 64 bit: \t{} ", float64);
    }

    // const float32:f32
    // const float64:f64;
}

pub mod textual {
    pub fn run() {
        let character = 'A';
      //  let str = "has";
        println!("unicode: \u{2518}");
        println!("char: {} ", character);
        println!("unicode: \u{2518}");
        println!("unicode: \u{2518}");
    }
}


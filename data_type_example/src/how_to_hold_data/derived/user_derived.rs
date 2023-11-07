pub mod structs {
    #[derive(Default)]
    struct Date {
        day: u8,
        month: u8,
        year: u32,
    }

    #[derive(Default)]
    pub struct user_defined_struct {
        name: String,
        age: i32,
        dob: Date,
        marital_status: bool,
    }
}

pub mod union {
    pub union person {
        id: u8,
        name: [char; 32],
        age: i32,
        height: i8,
        volume: [i8; 3],
    }
}

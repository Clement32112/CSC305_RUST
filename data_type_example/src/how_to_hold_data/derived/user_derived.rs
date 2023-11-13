#[allow(dead_code)]


pub mod structs {
    #[derive(Default,Debug)]
    
    struct Date {
        day: u8,
        month: u8,
        year: u32,
       
    }

    #[derive(Default,Debug)]
    pub struct UserDefinedStruct {
        name: String,
        age: i32,
        dob: Date,
        marital_status: bool,
    }
    pub fn run(){
        let example:UserDefinedStruct = UserDefinedStruct {
        name:String::from("Clement"),age:12,dob:Date{day:12,month:12,year:12},
        marital_status:true};
        println!("{:#?}", example);
        }
}

pub mod union {
  //#[warn(unused_variables)]
    #[allow(dead_code)]
    pub union Person {
        id: u8,
        name: [char; 32],
        age: i32,
        height: i8,
        volume: [i8; 3]
    }

    pub fn run(){
    let _example:Person = Person{volume: [12,1,3]};

    }
}

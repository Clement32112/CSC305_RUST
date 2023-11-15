
pub mod enums{
    enum Nums
    {
        ZERO,
        ONE,
        TWO,
        THREE
    }
}

#[allow(dead_code)]
pub mod structs {
    #[derive(Default,Debug)]
    struct Date {
        day: u8,
        month: u8,
        year: u32,

    }
    pub struct Point
    {
        pub x: i32,
        pub y:i32
    }
    struct Rectangle
    {
        top_left:Point,
        bottom_right:Point
    }
pub trait Movable
    {
        fn move_left(&mut self,steps:i32);

        fn display_coords(&self);

    }

    impl Movable for Point
    {
        fn move_left(&mut self,steps:i32) {
            self.x +=  steps;
        }
        fn display_coords(&self) {
            print!("x:{} ,y:{}",self.x,self.y)
        }
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


pub mod enums{
    #[allow(dead_code)]
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
        name:&'static str,
        top_left:Point,
        bottom_right:Point
    }
    impl Rectangle
    {
        fn default() -> Self 
        {
            Rectangle{ top_left:Point{x:0,y:0}, bottom_right:Point{x:10,y:10},name:"Default" }
        }
    }
    trait Shape
    {
        fn new(length:i32,width:i32,name:&'static str) ->Self;
        fn get_length(&self) -> i32;
        fn get_area(&self) -> i32;

    }
    impl Shape for Rectangle
    {
        fn new(length:i32,width:i32,name:&'static str) ->Self {

            Rectangle{top_left:Point{x:0,y:0}, bottom_right:Point{x:length,y:width},name}
        }
        fn get_area(&self) ->i32 {
            (self.bottom_right.x - self.top_left.x) * (self.bottom_right.y - self.top_left.y)
        }
        fn get_length(&self) -> i32 {
            self.bottom_right.x - self.top_left.x
        }
    }    

    impl PartialEq for Rectangle
    {
fn eq(&self, other: &Self) -> bool {
    self.get_area() == other.get_area()
} 
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
        let rec1:Rectangle = Rectangle::default();
        let rec2:Rectangle = Rectangle::new(12,12,"BON");
        println!("{}",rec2.get_area());
        println!("{}",rec2.get_length());
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

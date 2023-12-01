// If you aren't called stop complaining
#![allow(dead_code)] 


// Getting the comparson from the core of rust
use core::cmp::Ordering; 

pub enum Comp { //Enumerate Comparison
    LessThan, //0 
    GreaterThan, // 1
    Equal, // 2
}

pub enum Gender { //Enumerate Gender
    Male, // 0
    Female, // 1
}

// Metaprogramming. My understanding is  a implemetaion for coming with dynamic fucntions 
// An packced method injected in to the stuct if you will
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Unit struct. A struct only in name. Anything it has is "Inherited"
struct Unit;


//A tuple struct // An typed array from my perspective
struct Pair(i32, f32);  

//A struct representing coordinates system with typed parameters/values
struct Point {
    x: f32,
    y: f32,
}

// Nested structs 
struct Rectangle {
    // Takes a point struct as a value
    top_left: Point,

    bottom_right: Point,
}

// Run function for structs
pub fn run() {

    //declare a variable of type Person and assign values.
    let person = Person {
        // String from instatiates a String form a string slice. Like a packager
        name: String::from("Peter"),
        // Integer value
        age: 27,
    };
    // Pretty debug print. Keeping things as clean and tidy as possible in the terminal
    println!("{:#?}", person); // :? is for debug print  

    // Instantiate a unit struct
    let _unit = Unit;//As simple as that. If Unit implements some trait, then _unit will demand those implementations

    //declare a Point
    let point = Point { x: 10.3, y: 0.4 };

    //Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };
    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a let binding.
    let Point {
        x: left_edge,//left_edge here will be declared. If you use x:f32 only, x will be declared.
        y: top_edge,//top_edge here will be declared. If you use y:f32 only, y will be declared.
    } = point;
    dbg!(&left_edge,&top_edge);


    let _rectangle = Rectangle { //I used _ with rectangle to silence warning knowing that the variable is not used.
                                 //struct instantiation is an expression too as used below in Point
        top_left: Point {
            x: left_edge,//left_edge is available, thanks to the destructuring above
            y: top_edge,//top_edge is available, thanks to the destructuring above
        },
        bottom_right,
    };

    //Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    //Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    //Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}

// Behaviours given by shape trait
trait Shape {
    // Unique type that gets redefined by each struct type
    type Properties;
    // creates a new trait using it's unique propeties and a name
    fn new(properties:Self::Properties, name: &'static str) -> Self;
    // calculates area
    fn area(&self) -> f32;
    // calculates perimeter
    fn perimeter(&self) -> f32;
    // setting name static life time for borrow checker 
    fn set_name(&mut self, name: &'static str);
    // return name
    fn get_name(&self) -> &str;
    // perimeter comarison
    fn of_same_perimeter(&self,other:&Self) -> Option<Ordering>;

}

///Use Default to specify the availability of default instance creation without values passed for property
#[derive(Default, Debug, Clone)]
struct Rect {
    length: f32,
    width: f32,
    name: &'static str,
}

impl Rect {
    // These are unique traits given to only rects that cannot be used by others
    fn default() -> Self {Rect {length: 1.0,width: 1.0,name: "default_name"}}
    fn set_width(&mut self, width: f32) {self.width = width;}
    fn get_width(&self) -> f32 {self.width}
    fn set_length(&mut self, length: f32) {self.length = length;}
    fn get_length(&self) -> f32 {self.length}
}

impl Shape for Rect {
    // new defintion of propeties (length,width) 
    //unfortunately structs are not supported inside implementations yet
    type Properties = (f32,f32);
    // instancing a new rectange
    fn new(properties :Self::Properties, name: &'static str) -> Self {
        Rect {
            length:properties.0,// accessing the length property
            width:properties.1, // accessing the width property
            name, //same parameter same name 
        }
    }

    fn area(&self) -> f32 {
        self.length * self.width // L * B
    }
    fn perimeter(&self) ->f32{
        2.0 * (self.length + self.width ) // 2 (L + B)
    }


    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }
    fn of_same_perimeter(&self,other:&Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter())
    }    

}

//implement Partial Eq
impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area() // are areas the same
    }



    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }

    fn lt(&self, other: &Self) -> bool { self.area() < other.area() }
    fn le(&self, other: &Self) -> bool { self.area() <= other.area() }
    fn gt(&self, other: &Self) -> bool { self.area() > other.area() }
    fn ge(&self, other: &Self) -> bool { self.area() >= other.area() }
}


//A conversion implementation into String
//Expects a string slice with length, width, name, separated by commas
impl From<&'static str> for Rect {
    fn from(s: &'static str) -> Rect {
        let mut parts = s.split(',');
        let length = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let width = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Rect { length, width, name: &name }
    }
}


impl Into<Circle> for Rect{
    fn into(self) -> Circle { 
        let area:f32 =  self.get_width() * self.get_length(); 
        Circle {radius:  f32::sqrt(area / 3.142 as f32), name:self.name}
    }
}
impl Into<Triangle> for Rect{
    fn into(self) -> Triangle{
        let area:f32 = self.area();
        Triangle{height:f32::sqrt(area*2.0),base:f32::sqrt(area*2.0),name:self.name}
    }
}

pub fn run2() {
    let rectangle1 = Rect::default();


    println!("{}", rectangle1.length);
    println!("{}", rectangle1.width);
    println!("{}", rectangle1.name);

    let rectangle2 = Rect::new((1.0,3.0), "Rectangle2");
    let rectangle3 = Rect::from("4,5,Rectangle3");

    //Compare using PartialOrd
    let result1 = rectangle1.partial_cmp(&rectangle2);
    println!(" result1 = {:?}", result1);

    let result2 = rectangle1.le(&rectangle2);
    println!("result2 = {:?}", result2);

    //Compare using PartialEq
    let result3 = rectangle2.eq(&rectangle3);
    println!("result3 = {:?}", result3);


    let result4 = rectangle2.ne(&rectangle3);
    println!("result4 = {:?}", result4);

}

//Exercise
/*
   I need similar implementation for Circle and Triangle
   Besides Area, I need Perimeter and comparison on the basis of Perimeter
   In your submission, I need a comment against every line of code about what it is mearnt to achieve
   */

pub fn run3(){
    // creating new circle
    let rect1 = Rect::new((2.0,2.0),"Rectangle 1");
    println!("Perimeter of rect1 {}", rect1.perimeter());

    // creating new circle
    let circ1 = Circle::new(12.0,"Circle 1");
    println!("Perimeter of circ1 {}",circ1.perimeter());
    // circle from string
    let circ2 = Circle::from("16,Circle 2");
    println!("Perimeter of circ1 {:?}",circ2);
    let circ3 =Circle::default();
    // using the alias '<' 'lt' funtion
    if circ1 < circ2
    {
        println!("Circ1 is less than circ2")

    }
    // match comparison and unwrapping using match
    match circ1.partial_cmp(&circ3){
        Some(val) => {
            println!("comparing circ1  and circ3 :{:?}",val);
        },
        None => println!("Failed to compare")

    }
// Converting circle into triangle
    let tri3:Triangle = circ1.into();


    let tri1 = Triangle::new((2.0,3.0), "Triangle 1");
    println!("Perimeter of Triangle1 {}",tri1.perimeter());
    let tri2 = Triangle::from("12,12,Triangle 2");
    println!("Triangle from string {:?}",tri2);
    if tri1 < tri2
    {
        println!("tri1 is less than tri2");
    }
    else{println!("tri1 is not less than tri2");
}
    let perimeter_check = tri1.of_same_perimeter(&tri2).unwrap();
    println!("tri1 perimeter in comprison to tri2: {:?}",perimeter_check);


    match tri1.partial_cmp(&tri3){
        Some(val) => {
            println!("comparing tri1  and tri3 :{:?}",val);
        },
        None => println!("Failed to compare")

    }
}

#[derive(Default,Debug)]
struct Circle
{
    name: &'static str,
    radius:f32
}
impl Circle{
    fn get_radius(&self) -> f32{
        self.radius
    }

}

impl Shape for  Circle{

    type Properties = f32;
    fn new(properties:Self::Properties, name: &'static str) -> Self 
    {
        Circle{radius: properties,name}
    }
    fn area(&self) -> f32{
        std::f32::consts::PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f32
    {
        2.0 * std::f32::consts::PI * self.radius
    }
    fn set_name(&mut self, name: &'static str){
        self.name = &name;
    }
    fn get_name(&self) -> &str{
        self.name}
    fn of_same_perimeter(&self,other:&Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter())
    }
}

impl From<&'static str> for Circle {
    fn from(s: &'static str) -> Circle {
        let mut parts = s.split(',');
        let radius = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Circle { radius, name: &name }
    }
}

impl PartialEq for Circle
{
    fn eq(&self, other: &Self) -> bool {
        // calculate area and compare
        self.area() == other.area()
    }
    // not equals to 
    fn ne(&self, other: &Self) -> bool {
        self.area() != other.area()
    }
}

impl PartialOrd for Circle
{
    // simple partial_cmp for Circle
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area()) 
    }
    // less than <
    fn lt(&self, other: &Self) -> bool { self.area() < other.area() }
    // less than or equal to = 
    fn le(&self, other: &Self) -> bool { self.area() <= other.area() }
    // greater than >  
    fn gt(&self, other: &Self) -> bool { self.area() > other.area() }
    // greather than > or equal to =
    fn ge(&self, other: &Self) -> bool { self.area() >= other.area() }
}


impl Into<Rect> for Circle{
    // Converting a circle into a rectangle
    fn into(self) -> Rect { 
        let area:f32 =  self.area(); 
        Rect {length:f32::sqrt(area) ,width:f32::sqrt(area), name:self.name}
    }
}
impl Into<Triangle> for Circle{
    // Converting a circle into a triangle
    fn into(self) -> Triangle{
        let area:f32 = self.area();
        Triangle{height:f32::sqrt(area*2.0),base:f32::sqrt(area*2.0),name:self.name}
    }
}

#[derive(Debug)] // implement debug to visualize parameters
struct Triangle
{
    height: f32,
    base:f32,
    name :&'static str

}

impl Triangle{
    // returns height of triangle
    fn get_height(&self) -> f32{
        self.height
    }
    // setter for height
    fn set_height(&mut self,height:f32) {
        self.height = height;
    }
    // getter for base
    fn get_base(&self) -> f32{
        self.base
    }
    // setter for base
    fn set_base(&mut self,base:f32){
        self.base = base;
    }
    //get hypotenuse of triangle
    fn hypotenuse(&self) -> f32{
        f32::sqrt(self.height *self.height + self.base + self.base)
    }
}

// implementation of Shape for Triangle
impl Shape for Triangle{
    // Height and base assumed righ angle
    type Properties = (f32,f32);
    // New instatiation
    fn new(properties:Self::Properties, name: &'static str) -> Self{
        Triangle{height: properties.0,base:properties.1,name}
    }
    // Area 1/2 * height * base
    fn area(&self) -> f32
    {
        0.5 * self.height *self.base 
    }
    // perimeter sum height base and hypotenuse
    fn perimeter(&self) -> f32{
        self.height + self.base + self.hypotenuse()
    }

    fn set_name(&mut self, name: &'static str){
        self.name = name;
    }
    fn get_name(&self) -> &str{self.name}

    fn of_same_perimeter(&self,other:&Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter())
    }

}
impl From<&'static str> for Triangle {
    //implement from funtion taking a string "height, base, name" 
    fn from(s: &'static str) -> Triangle {
        // Seperate where you see  commas
        let mut parts = s.split(',');

        // Get the first part
        let height = match parts.next() {
            // if has a value convert to string and store
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0, // Else 0
        };
        // Get next session
        let base = match parts.next() {
            // if has a value convert to string and store
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0, // Else 0
        };

        // Get next session
        let name = match parts.next() {
            // No need to parse it's already a string string
            Some(val) => val,
            None => "", // else empty string
        };
        // Return triangle with parsed parameters
        Triangle { height, base, name: &name }
    }

}

// Partial cmp with triangle
impl PartialEq for Triangle
{
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }
    fn ne(&self, other: &Self) -> bool {
        self.area() != other.area()
    }
}
impl PartialOrd for Triangle
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area()) 
    }

    fn lt(&self, other: &Self) -> bool { self.area() < other.area() }
    fn le(&self, other: &Self) -> bool { self.area() <= other.area() }
    fn gt(&self, other: &Self) -> bool { self.area() > other.area() }
    fn ge(&self, other: &Self) -> bool { self.area() >= other.area() }
}

impl Into<Rect> for Triangle{
    fn into(self) -> Rect { 
        let area:f32 =  self.area(); 
        Rect {length:f32::sqrt(area) ,width:f32::sqrt(area), name:self.name}
    }
}

impl Into<Circle> for Triangle{
    fn into(self) -> Circle { 
        let area:f32 =  self.area(); 
        Circle {radius:  f32::sqrt(area / 3.142 as f32), name:self.name}
    }
}

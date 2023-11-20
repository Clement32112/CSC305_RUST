
pub mod lifetimes
{

    pub fn life_time_scope()
    {
        let mut x:i32 = 1;
        {
            // Y is borrowing the value of x
            let y = &x;

            // x +=1; here would throw an error

            println!("x of value: {}, has been borrowed",x);
            println!("y of value {}, has borrowed from x",y);

            x+=1;
            // y returns ownership back to x as it i no longer called again
            println!("x += 1 : {} it works as y is no longer called",x);
        }

    }
}

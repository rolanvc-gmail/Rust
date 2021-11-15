mod to_do;

use to_do::structs::done::Done;
use to_do::structs::pending::Pending;


fn main() {
    let done: Done = Done::new("shopping");
    println!("Title is:  {}", done.super_struct.title);
    println!("Status is: {}", done.super_struct.status);

    let pending = Pending::new("laundry");
    println!("Title is:  {}", pending.super_struct.title);
    println!("Status is: {}", pending.super_struct.status);

}
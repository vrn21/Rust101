fn main() {
    //assigning variable
    //let a = 5;
    //let mut b = 6;

    //println!("") ! this shows that this is a macro
    //println!("{a}"); var a wil be print
    //println!("hy adsfgh {:?}",a);

    //defining a fn
    //fn add(a : i32, b : i32) ->i32 {fn statements}
    //here i32 shows integer of 32 bit size. -> i32 shows that the output is of that datatype

    //loop while loop is also there
    /*let mut i = 0;
    loop {
        println!("the number is {i}");
        if i == 5 {
            break;
        }
        i = i + 1;
    }*/
    // let mut name = "Varun";
    // println!("My first name is {name}");
    // name = "KV";
    fn fname() {
        println!("Varun");
    }
    fn lname() {
        print!("KV");
    }
    fname();
    lname();
    println!("hi rust running : )");
}

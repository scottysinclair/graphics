

pub(crate) fn pass_by_copy() {
    fn inc(mut x: i32) { x = x + 1; }
    let mut x = 1;
    x = 2;
    println!("{}" , x);
    inc(x);
    println!("{}" , x); //pass by copy, x has not changed
}

pub(crate) fn move_ownership_through_assignment() {
    let s1 = String::from("hello");  //create a string owned by s1
    let s2 = s1;                       //transfer ownership to s2
    //println!("{}, world!", s1); //fails to compile, s2 now owns the string
}

pub(crate) fn move_ownership_through_function_call() {
    fn printit(x :String) { println!("{}, world!", x); }
    let s1 = String::from("hello");  //create a string owned by s1
    printit(s1);  //move ownership into the function
    //println!("{}, world!", s1); //fails to compile, function owned the string and then dropped it
}

pub(crate) fn give_and_take_ownership() {
    fn give() -> String { String::from("yours") }
    fn take_and_give(s: String) -> String { s }

    let s1 = give();
    let s2 = take_and_give(s1);
    println!("{}, world!", s2);
    //println!("{}, world!", s1);  //fails to compile s1 moved into function and then out to s2
}

pub(crate) fn borrow_string() {
    fn calculate_length(s :&String) -> usize { s.len() }
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

pub(crate) fn borrow_mut_string() {
    fn append(s : &mut String) { s.push_str(" world"); }
    let mut s1 = String::from("hello");
    append(&mut s1);
    println!("appened {}", s1);
}

pub(crate) fn two_mutable_references_not_allowed() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    //let r2 = &mut s; compile error this is not allowed
    //println!("{}, {}", r1, r2);
}

pub(crate) fn two_mutable_references_not_symultanious() {
    let mut s = String::from("hello symultanious");
    let r1 = &mut s;
    println!("{}", r1); // r1 is implicitly dropped
    let r2 = &mut s;  //r2 can safely mutably borrow
    println!("{}", r2);
}
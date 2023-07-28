use crate::reference::*;
use crate::graphics1::*;
use crate::graphics2::*;


pub mod reference;
pub mod graphics1;
pub mod graphics2;

//https://github.com/jeremyletang/rust-sfml/blob/master/examples/music-stream.rs



fn main() {
    pass_by_copy();
    move_ownership_through_assignment();
    move_ownership_through_function_call();
    borrow_string();
    borrow_mut_string();
    two_mutable_references_not_allowed();
    two_mutable_references_not_symultanious();


    //main1();
    main2();
}





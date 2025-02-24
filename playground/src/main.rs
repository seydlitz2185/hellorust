pub mod ep1_basic;
pub mod ep2_upgrade;

pub use crate::ep1_basic::*;
pub use crate::ep2_upgrade::*;
 fn main() {
    /*
    let foo;
    foo = 13;
    let mut bar;
    bar = "14";
    bar = "13";
    println!("{foo}\n{bar}");
    */
    basic::ownership();
    basic::shadow();
    basic::slice();
    enum_demo::pattern_matching();
    generics_demo::deref_wrapper();
    option_pattern_demo::divide_4(10);
    option_pattern_demo::try_divide_2(10);
    option_pattern_demo::try_divide_2(11);
}

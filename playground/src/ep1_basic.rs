
pub mod basic{
    pub fn print_i32(i:i32){
        println!("{}",i);
    }
    pub fn print_str(s:String){
        println!("{}",s);
    }
    pub fn slice (){
        let mut int_slice : [i32;5] = [1,2,3,4,5];

        let ref_slice : &[i32] = &int_slice[1..4];

        ref_slice.iter().for_each(
            |i
            | print!("{} ",i));
        println!();
        let ref_slice : &[i32] = &int_slice[1..=4];
        ref_slice.iter().for_each(
            |i
            | print!("{} ",i));
        println!();
    }
    pub fn shadow(){
        let  mut str = "Hi, Mom".to_string();
        print_str(std::mem::take(&mut str));
    //   let  mut str = "Hi, Dad".to_string();
        print_str(str);
    }


    pub fn ownership () ->(){
        let mut var :i32 =5;
        
        println!{"Mutable variable var = {}",var};
        let ref1 = &var; 
        let ref2 = &var; //可以对一个变量声明定义多个引用

        let mutref = &mut var; //若不使用ref1或者2，不报错，使用则报错
        println!("var has mutable reference mutref with value = {}",mutref);
        
        *mutref = 10;
        println!{"Mutable variable var is {} now",var};
}
}

pub mod enum_demo{
    #[derive(Debug)]
    enum SomeType{
        Str(String),
        Int(i32),
        Tuple(i32,i32),
    }

    pub(crate)fn pattern_matching(){
        let some = SomeType::Str("Hi Mom".to_string());
        match some{
            SomeType::Str(s) => println!("String: {}",s),
            SomeType::Int(i) => println!("Integer: {}",i),
            SomeType::Tuple(i,j) => println!("Tuple: {} {}",i,j),
        }
    }
}



pub mod generics_demo{
    use std::ops::{Deref,DerefMut};
        //泛型类Wrapper
    struct Wrapper <T>{
        value:T,
    }
    
    //为Wrapper类型不可变变量实现Deref trait，自动解引用
    impl<T> Deref for Wrapper<T>{
        type Target = T;
        fn deref(&self)-> &Self::Target{
            &self.value
        }
    }
    
    //为Wrapper类型可变变量实现Deref trait，自动解引用
    impl<T> DerefMut for Wrapper<T>{
        fn deref_mut(&mut self)-> &mut Self::Target{
            &mut self.value
        }
    }
    use crate::ep1_basic::basic::{print_i32,print_str};
    
    pub fn deref_wrapper(){
        //测试泛型类自动解引用
        let wrapper_int = Wrapper{value:42};
        //recall：pass by value for int type
        print_i32(*wrapper_int);
        let wrapper_str = Wrapper{value:"Hi Dad".to_string() };
        //recall：pass by copy for str type
        print_str((*wrapper_str).clone());
    }    
}


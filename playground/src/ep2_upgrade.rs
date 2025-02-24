pub mod option_pattern_demo{
    pub fn divide_2 (i:i32)->Option<i32>{
        if i%2==0{
            Some(i/2)
            //Ok(i/2)
        }else{
            None
            //Err("err")
        }
    }
    pub fn divide_2_res (i:i32)->Result<i32,String>{
        if i%2==0{
            Ok(i)
        }else{
            Err("oops".to_string())
        }
    }
    /*
    检验一个数是否可被4整除
    可以先检验是否可被2整除，
    再检验除它以2后的商可否被2整除
    */
    pub fn divide_4 (i:i32)->Option<i32>{
        //简洁写法
        /*let res  = divide_2(i)?;
        divide_2(res);*/
        let res = divide_2(i);
        match res{
            Some(i) => divide_2(i),
            None => None,
        }    
    }

    pub fn try_divide_2(i:i32){
        if let Ok(res) = divide_2_res(i){
            println!("Result:{} is divisible by 2",i);
        } else  if let  Err(e)  = divide_2_res(i){
            println!("Error:{},{} is not divisible by 2",e,i);
        }
        //Err(_) place holder
    }
}
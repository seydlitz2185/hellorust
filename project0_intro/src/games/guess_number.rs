use rand::Rng;
pub(crate) fn init() ->u8{
    let mut rng = rand::rng();
    let num: u8 = rng.random();
    return (num %100) + 1;
}

pub(crate) fn guess (guess:u8,num :u8) -> bool{
    if num > guess {
        println!("Too,small! Try again");
    }else if num < guess{
        println!("Too big! Try again");
    }else {
        println!("Cheers! You have guessed the right number {}",num);
        return true;
    }
    return  false;
}

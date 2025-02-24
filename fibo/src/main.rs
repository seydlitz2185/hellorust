fn fibo( i: i32,j:i32 ) -> i32{
    let k = i + j;
    print!("{} ",k);
    if k > 1000 {
        return 0 ;
    }
        return fibo(j,k);
}
fn main() {
    fibo(0i32,1i32);
}

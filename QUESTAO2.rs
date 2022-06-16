fn main() {
    
    let num = 13;
    let mut aux = 0;
    
    while fib(aux) < num {
        aux +=1;
    }
    if fib(aux) == num{
        
        println!("{} é um numero da sequencia",num);
    }else{
        println!("{} não é um numero da sequencia",num);
    }
    
}
fn fib(num: i32 ) -> i32{
    if num <= 1{
        return num
    }
    fib(num-1) + fib(num-2)
}
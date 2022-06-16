fn main() {
    let strInit: String = "Desafio Target!!!".to_owned();
    let mut strInv: String = "".to_owned();
    
    for c in strInit.chars().rev(){
        strInv = strInv + &c.to_string();
    }    
    println!("{}",strInv);
}
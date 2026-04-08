fn main() {
    println!("Hello, world!");
    pattern_matcheing();
    erros();
}
fn pattern_matcheing(){
 for x in 1..=20{
     println!("{} : {}", x , match x {
         1 => "Pouco",
         2|3 => "um pouco",
         4..10 => "Um bocado",
         _ if x % 2 == 0 => "Uma boa quantidade",
         _ => "muito"

     });
 }
}
fn erros() {
    //panic!("Erro proposital");
    match resultado() {
        Ok(s) => println!("String de sucesso foi essa = {}", s),
        Err(numero) => println!("Codigo de erro = {}", numero)
    }
}
fn resultado() -> Result<String , u8>{
    Err(42)
}

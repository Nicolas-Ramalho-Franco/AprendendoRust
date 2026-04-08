fn main() {
    repticao();
    ownership();
}
fn repticao(){
    let multiplicador :u8 = 5;
    let mut contador :u8 = 0;

    while contador <= 10 {
        contador +=1;
        if contador == 5 {
            continue;
        }
        println!("{} X {} = {}",multiplicador, contador, multiplicador*contador );
    }
    
    contador = 0;
    loop {
        contador += 1;
        println!("{} X {} = {}",multiplicador, contador, multiplicador*contador );
        if contador == 10 {
            break;
        }
    }
    for i in 1..11{
        println!("{} X {} = {}",multiplicador, i, multiplicador*i );
    }
}
fn ownership(){
    let mut uma_string = String::from("Nicolas");
    rouba(& mut uma_string);

    println!("{}", uma_string);
}
fn rouba(string: &mut String) {
    string.push_str(" Ramalho");
    println!("{}", string);
}
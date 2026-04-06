const PI: f64 = 3.14;
static  variavel_global: i32 =1;

fn sombra(){
    let a =123;
    {
        let b =456;
        println!("Dentro , b = {}", b);
        let a=777;
        println!("Dentro ,a = {}", a);
    }
    println!("a = {}", a);
}

fn escopo(){
    println!("PI: {}", PI);
    print!("Variavel global:  {} ", variavel_global);

    let variavel = 300;
    println!("variavel = {}, tamanho ={} bytes", variavel,std::mem::size_of_val(&variavel));

    let decimal = 2.5;
    println!("decimal = {}", decimal);

    let boolean = true;
    println!("tamanho boleano= {} bytes", std::mem::size_of_val(&boolean) );

    let letra :char = 'x';
    println!("Tamannho letra= {} bytes", std::mem::size_of_val(&letra) );
}
fn main() {
    sombra();
    escopo();
}

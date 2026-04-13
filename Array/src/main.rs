fn main() {
    let notas =[6.5;4]; // 4 notas de 6.5
    let inteiro:usize = 0;//isso pega o indice q eu quero exibir
    println!("{}", notas[inteiro]);

    for indice in 0..notas.len(){
        println!(" sua nota {} e :{}", indice +1, notas[indice]);
    }

    matriz()
}
fn matriz(){
    let matriz =[
        [0.0, 1.2 , 0.1],
        [1.3 ,0.3 , 1.4],
    ];

    for linhas in matriz{
        for intem in linhas{
            println!("item ={}", intem);
        }
    }
}
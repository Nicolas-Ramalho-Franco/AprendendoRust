fn main() {
    vectores();
}
fn vectores() {
    let mut notas :Vec<f32> =Vec::with_capacity(4);//Vec::with_capacity(4) que voce defini um valor ou Vec::new(); ou pode usar assim vec![10.0,4.6,2.3] que substitui o valor que esta com o .push
    println!("Capacidades de notas ={}", notas.capacity());
    notas.push(10.0);
    notas.push(4.6);
    notas.push(2.3);
    println!("{:?}", notas);

    notas.push(10.0);
    println!("{:?}", notas);
    println!("Capacidades de notas ={}", notas.capacity());

    println!("Nota 1 :{}", notas[0]);
    println!("Nota 6 :{}", match notas.get(3) {
        Some(n) => *n,
        None => 0.0
    });

    //while let Some(nota) = notas.pop(){
    //    println!("Nota removida = {:?}",notas.pop());//pega o ultimo valor retorna e apaga ele depois
    //    println!("nota :{:?}",nota);
    //}

    for mota in &notas{
        println!("nota = {}", mota);
    }

    println!("nota :{:?}",notas);

}
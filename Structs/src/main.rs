fn main() {
    conta_corrente()
}
struct Titular{
    nome:String,
    sobrenome:String

}
struct Conta{
    titular: Titular,
    saldo:f64
}
impl Conta{//isso funciona para vc implmentar algo dentro de alguma struct //da apara usar em enums
    fn sacar(&mut self , valor:f64){// aqui fica a função e sempre vai esse self
        self.saldo-=valor
    }
}

fn conta_corrente() {
    let mut conta:Conta = Conta{// teve que colocar o mut por conta do impl
      titular:Titular{nome:String::from("Nicolas") , sobrenome:String::from("Ramalho")},
      saldo:100.0
    };

    conta.sacar(50.0);
    println!("Esse e o nome do titular {} {} e esse e o saldo {}" ,
             conta.titular.nome,conta.titular.sobrenome ,conta.saldo);
}
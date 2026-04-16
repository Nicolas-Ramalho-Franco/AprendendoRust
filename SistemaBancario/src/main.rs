fn main() {
    println!("Ola bem vindo ao sistema bancario do Nicolas");
    escolaDePlano();
}
fn escolaDePlano(){
    let escolha = "Tranferencia";  // "Receber" ou  "Tranferencia"
    let funcao = match escolha {
        "Receber" => receber(),
        "Trasferencia" =>transferir(),
        _=> erro()
        
    };
}
fn erro(){
    println!("Fez errado ai lerdão");
}
fn receber(){
    let mut conta:Conta = Conta{
        titular:String::from("Nicolas"),
        saldo:100.0
    };
    conta.depositar(100.0);
    println!("Seu novo saldo e {}", conta.saldo);
}
fn transferir(){
    let mut conta:Conta = Conta{
        titular:String::from("Nicolas"),
        saldo:100.0
    };
    conta.sacar(100.0);
    println!("Seu novo saldo e {}", conta.saldo);
}
struct Conta {
    titular:String,
    saldo:f64
}
impl Conta {
    fn sacar(&mut self , valor:f64){// aqui fica a função e sempre vai esse self
        self.saldo-=valor;
        println!("Seu novo saldo e {}", self.saldo);
    }
}
impl Conta {
    fn depositar(&mut self, valor:f64){
        self.saldo += valor;
        println!("Seu novo saldo e {}", self.saldo);
    }
}

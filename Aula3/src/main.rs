fn main() {
    condicao();
}
fn condicao() {
    let idade:u8 = 9;
    let responsavel_autorizou = false;
    let eh_maior = idade >=18;

    if eh_maior {
        println!("Pode entrar na balada");
    }else if idade >=16 && responsavel_autorizou {
        println!("Pode entrar na balada com documentação");
    }
    else{
        println!("Não pode entrar na balada");
    }

    let condicao = if eh_maior {"Maior"} else {"Menor"};
    println!("E {} de idade",condicao);
}

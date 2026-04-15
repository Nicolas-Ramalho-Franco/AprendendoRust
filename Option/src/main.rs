fn main() {
    conteudo_opcional();
}
fn conteudo_opcional() {
    let conteudo_arquivo =ler_arquivo(String::from(""));

    match &conteudo_arquivo {
        Some(valor) => println!("Conteudo arquivo: {}", valor),
        None => println!("Arquivo sem conteudo")
    };
    println!("{:?}", conteudo_arquivo);

    if let Some(valor) = conteudo_arquivo {
        println!("Agora tenho certeza que tenho o valor : {}", valor);
    }
}

fn ler_arquivo(caminho_arquivo:String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
    //None
}
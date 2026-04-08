
fn main() {
    let linguagem = "oi";
    let proposito = match linguagem {
        "PhP" => "Web",
        "Kotlin" => "Android",
        "PY" => "Data Science",
        _ => "Não sei"
    };
    if proposito == "Não sei" {
        println!("Digite alguma variavel certa")
    }else {
        println!("O proposito de {} eh {}", linguagem, proposito);
    }
}

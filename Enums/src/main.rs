fn main() {
    println!("E fim de semana {}" , fim_de_semana(diadasemana::Domingo));
    println!("{:?}", cores())
}
enum Color {
    Red,
    Green,
    Blue,
    RGBcolor(u8, u8, u8),
    CymkColor{cyan: u8, magenta: u8, yellow: u8, black: u8}
}
fn cores(){
    let cor = Color::CymkColor {cyan: 1, magenta: 2, yellow: 3, black: 255};

     println!( "Cor {} ",match cor{
        Color::Red => "Red",
        Color::Green =>"Green",
        Color::Blue => "Blue",
        Color::RGBcolor(0, 0, 0)|Color::CymkColor {cyan: _, magenta: _, yellow: _, black: 255} =>"Isso e preto",
        Color::RGBcolor(_, _, _) => "Desconhecida",
        Color::CymkColor {cyan: _, magenta: _, yellow: _, black: _} =>"CYMK desconhecida"
    });
}
enum diadasemana{
    Domingo,
    segunda,
    terca,
    quarta,
    quinta,
    sexta,
    sabado
}
fn fim_de_semana(dia_da_semana:diadasemana) -> bool{

    match dia_da_semana {
        diadasemana::Domingo|diadasemana::sabado =>true,
        _ => false
    }
}

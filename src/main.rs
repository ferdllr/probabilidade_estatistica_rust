mod calc;
use calc::Calc;
fn main() {
    println!("===========");
    println!("--probabilidade--");
    let dados_possiveis = [1, 2, 3, 4, 5, 6];
    println!(
        "a probabilidade de cair um numero par é de: {}%",
        Calc::probabilidade(&dados_possiveis, &|n| n % 2 == 0) * 100.0
    );
    println!(
        "a probabilidade de cair um numero impar é de: {}%",
        Calc::probabilidade(&dados_possiveis, &|n| n % 2 != 0) * 100.0
    );
    println!(
        "a probabilidade de cair um numero maior que 4 é de: {}%",
        Calc::probabilidade(&dados_possiveis, &|n| n > 4) * 100.0
    );

    println!("===========");
    println!("--media--");
    let notas = [5.1, 8.5, 10.0, 5.5, 3.2, 7.9, 8.2, 2.2];
    println!(
        "a média das notas é: {:.2}",
        Calc::media_aritimetica(&notas)
    );

    println!("===========");
    println!("--mediana--");
    let salarios = [1000.0, 1200.0, 850.0, 1540.0, 20.0, 2200.0, 1950.0, 5000.0];
    println!("a mediana é: {}", Calc::mediana(&salarios));

    println!("===========");
    println!("--moda--");
    let intnums = [2, 5, 3, 3, 8, 7, 10, 120, 5, 3, 5, 5];
    println!("existe: a moda é: {:?}", Calc::moda(&intnums));
    let semmoda = [1, 2, 3, 4, 5, 6, 7];
    println!("não existe: a moda é: {:?}", Calc::moda(&semmoda));

    println!("===========");
    println!("--amplitude--");
    let amp_array1 = [7.0, 7.0, 7.0, 7.0, 7.0];
    let amp_array2 = [5.0, 6.0, 7.0, 8.0, 9.0];
    println!("1a lista: amplitude: {:?}", Calc::amplitude(&amp_array1));
    println!("2a lista: amplitude: {:?}", Calc::amplitude(&amp_array2));

    println!("===========");
    println!("--variancia--");
    println!("variancia das notas: {:?}", Calc::variancia(&amp_array2));
}

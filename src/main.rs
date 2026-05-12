fn main() {
    // Punctul de intrare al programului.
    // Apelează pe rând exemplele/exercițiile pentru secțiunile parcurse.
    lunches_1_1();
    lunches_1_2();
    lunches_1_3();
}

fn lunches_1_1() {
    // Secțiunea 1.1: String mutabil + adăugare caracter + afișare.
    let mut my_name: String = "Dave".to_string();
    my_name.push('!');
    println!("{}", my_name);
}

fn lunches_1_2() {
    // Secțiunea 1.2: exemple de comentarii în Rust.
    // Programele Rust pornesc din fn main().
    // Codul este scris într-un bloc delimitat de { și }.

    let some_number = 100; // Compilatorul ignoră tot ce este în dreapta lui //
    let some_number /*: i16*/ = some_number;
    println!("some_number = {}", some_number);
}

fn lunches_1_3() {
    // Secțiunea 1.3: tipuri primitive, conversii și diferența bytes vs caractere.

    // Conversie sigură exemplificată din u8 în char (100 -> 'd').
    let my_number: u8 = 100;
    println!("100 as char = {}", my_number as char);

    // char are mereu 4 bytes în Rust (Unicode scalar value).
    println!("Size of a char: {}", std::mem::size_of::<char>());

    // Pentru string-uri, len() întoarce numărul de bytes.
    println!("Size of a: {}", "a".len());
    println!("Size of ß: {}", "ß".len());
    println!("Size of 国: {}", "国".len());

    let str1 = "Hello!";
    let str2 = "안녕!";

    // chars().count() întoarce numărul de caractere (nu bytes).
    println!(
        "str1 is {} bytes and {} characters.",
        str1.len(),
        str1.chars().count()
    );
    println!(
        "str2 is {} bytes and {} characters.",
        str2.len(),
        str2.chars().count()
    );
}


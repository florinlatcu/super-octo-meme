fn main() {
    // Punctul de intrare al programului.
    // Rulează exercițiile pentru capitolul 1 din cartea "Month of Lunches".
    chapter1::run();
}

mod chapter1 {
    pub fn run() {
        // ===== Capitolul 1: Some basics =====
        lunches_1_1();
        lunches_1_2();
        lunches_1_3();
        lunches_1_4();
        lunches_1_5();
    }

    fn lunches_1_1() {
        // ----- 1.1 Introducing Rust -----
        // String mutabil + adăugare caracter + afișare.
        let mut my_name: String = "Dave".to_string();
        my_name.push('!');
        println!("{}", my_name);
    }

    fn lunches_1_2() {
        // ----- 1.2 Comments -----
        // Programele Rust pornesc din fn main().
        // Codul este scris într-un bloc delimitat de { și }.

        let some_number = 100; // Compilatorul ignoră tot ce este în dreapta lui //
        let some_number /*: i16*/ = some_number;
        println!("some_number = {}", some_number);
    }

    fn lunches_1_3() {
        // ----- 1.3 Primitive types -----
        // Tipuri primitive, conversii și diferența bytes vs caractere.

        // Conversie exemplificată din u8 în char (100 -> 'd').
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

    fn lunches_1_4() {
        // ----- 1.4 Type inference -----
        // Rust alege tipuri implicite când nu specificăm explicit.

        let inferred_int = 8;
        let explicit_u8: u8 = 10;
        let suffix_u8 = 10u8;
        let readable_i32 = 100_000_000_i32;

        println!(
            "inferred_int = {}, explicit_u8 = {}, suffix_u8 = {}, readable_i32 = {}",
            inferred_int, explicit_u8, suffix_u8, readable_i32
        );

        // Underscore-urile nu schimbă valoarea numerică, doar lizibilitatea.
        let number = 0________u8;
        let number2 = 1___6______2____4______i32;
        println!("number = {}, number2 = {}", number, number2);
    }

    fn lunches_1_5() {
        // ----- 1.5 Floats -----
        // Float-urile sunt numere cu punct zecimal: f32 și f64.

        let my_float = 5.;
        println!("my_float = {} (implicit f64)", my_float);

        let my_float: f64 = 5.0;
        let my_other_float: f32 = 8.5;
        let third_float = my_float + my_other_float as f64;
        println!("f64 + (f32 as f64) = {}", third_float);

        let inferred_left = 5.0;
        let inferred_right = 8.5;
        let inferred_sum = inferred_left + inferred_right;
        println!("inferred_sum (f64 + f64) = {}", inferred_sum);

        let forced_f32_left: f32 = 5.0;
        let forced_f32_right = 8.5;
        let forced_f32_sum = forced_f32_left + forced_f32_right;
        println!("forced_f32_sum (f32 + f32) = {}", forced_f32_sum);
    }
}


// Exerciții pentru Capitolul 3
// Template unitar pentru fiecare exercițiu:
// - Dificultate
// - Obiectiv
// - Criterii de succes
// - Concepte folosite
// - Capcană frecventă

const EXERCISE_SELECTOR: Option<u8> = None;

pub fn run() {
    println!("\n=== Exerciții: Capitolul 3 ===");
    println!("Selector curent: {:?}", EXERCISE_SELECTOR);

    match EXERCISE_SELECTOR {
        Some(ex_id) => run_by_id(ex_id),
        std::option::Option::None => run_recommended_path(),
    }
}

fn run_recommended_path() {
    println!("\n--- Traseu recomandat (Bază -> Combinat -> Mini-test) ---");
    for ex_id in [1u8, 2, 3, 4, 5, 6, 7, 11] {
        run_by_id(ex_id);
    }
}

fn run_by_id(ex_id: u8) {
    match ex_id {
        1 => exercitiu_3_1_arrays_vectors_tuples(),
        2 => exercitiu_3_2_arrays_index_slice(),
        3 => exercitiu_3_3_vectors_capacity(),
        4 => exercitiu_3_4_tuple_destructuring(),
        5 => exercitiu_3_5_if_else(),
        6 => exercitiu_3_6_match_exhaustiv(),
        7 => exercitiu_3_7_match_guards(),
        8 => exercitiu_3_8_loop_while_for(),
        9 => exercitiu_3_9_break_cu_valoare(),
        10 => exercitiu_3_10_provocare_mini_program(),
        11 => mini_test_final_capitol_3(),
        _ => println!("[Capitolul 3] Exercițiul {} nu există.", ex_id),
    }
}

fn exercitiu_3_1_arrays_vectors_tuples() {
    println!("\n[Cap.3][Ex.1][Bază] Arrays, vectors, tuples");
    // Obiectiv: declară un array, un vector și un tuple.
    // Criterii de succes: 1) toate trei declarate corect, 2) afișare clară.
    // Concepte folosite: [T;N], Vec<T>, tuple.
    // Capcană frecventă: amestec de tipuri nepermise în array/vec.
}

fn exercitiu_3_2_arrays_index_slice() {
    println!("\n[Cap.3][Ex.2][Bază] Index și slice pe array");
    // Obiectiv: accesează prin index și creează două slice-uri.
    // Criterii de succes: 1) index valid, 2) slice exclusiv, 3) slice inclusiv.
    // Concepte folosite: indexing, slicing, range.
    // Capcană frecventă: out-of-bounds.
}

fn exercitiu_3_3_vectors_capacity() {
    println!("\n[Cap.3][Ex.3][Bază] Capacitate vector");
    // Obiectiv: urmărește capacity înainte și după push.
    // Criterii de succes: 1) afișare capacity inițial, 2) după mai multe push-uri, 3) observație realocare.
    // Concepte folosite: Vec::new, push, capacity.
    // Capcană frecventă: confuzia între length și capacity.
}

fn exercitiu_3_4_tuple_destructuring() {
    println!("\n[Cap.3][Ex.4][Bază] Destructurare tuple");
    // Obiectiv: destructurează tuple și folosește doar o parte din valori.
    // Criterii de succes: 1) pattern corect, 2) underscore pentru valori ignorate.
    // Concepte folosite: tuple destructuring.
    // Capcană frecventă: pattern cu număr greșit de elemente.
}

fn exercitiu_3_5_if_else() {
    println!("\n[Cap.3][Ex.5][Bază] if / else if / else");
    // Obiectiv: clasifică un număr (pozitiv/negativ, par/impar).
    // Criterii de succes: 1) condiții clare, 2) ramuri complete.
    // Concepte folosite: expresii booleene, operatori logici.
    // Capcană frecventă: confuzie `=` vs `==`.
}

fn exercitiu_3_6_match_exhaustiv() {
    println!("\n[Cap.3][Ex.6][Bază] Match exhaustiv");
    // Obiectiv: match cu wildcard `_` pentru cazurile neacoperite explicit.
    // Criterii de succes: 1) toate ramurile acoperite, 2) output clar per caz.
    // Concepte folosite: match, wildcard.
    // Capcană frecventă: match neexhaustiv.
}

fn exercitiu_3_7_match_guards() {
    println!("\n[Cap.3][Ex.7][Combinat] Match guards");
    // Obiectiv: clasifică un tuple numeric folosind guard-uri.
    // Criterii de succes: 1) cel puțin două guard-uri, 2) ramură implicită `_`.
    // Concepte folosite: match guards, tuple patterns.
    // Capcană frecventă: condiții care se suprapun neintenționat.
}

fn exercitiu_3_8_loop_while_for() {
    println!("\n[Cap.3][Ex.8][Combinat] loop, while, for");
    // Obiectiv: scrie câte un exemplu scurt pentru fiecare tip de buclă.
    // Criterii de succes: 1) fiecare buclă are criteriu clar de oprire.
    // Concepte folosite: loop, while, for, range.
    // Capcană frecventă: buclă infinită accidentală.
}

fn exercitiu_3_9_break_cu_valoare() {
    println!("\n[Cap.3][Ex.9][Combinat] break cu valoare");
    // Obiectiv: întoarce o valoare din `loop` cu `break valoare`.
    // Criterii de succes: 1) valoare calculată în loop, 2) preluată în variabilă externă.
    // Concepte folosite: loop expression.
    // Capcană frecventă: tipuri incompatibile în ramuri.
}

fn exercitiu_3_10_provocare_mini_program() {
    println!("\n[Cap.3][Ex.10][Mini-provocare] Mini program");
    // Obiectiv: mini-program care raportează statistici simple pentru o listă de numere.
    // Criterii de succes: 1) folosești vec + loop, 2) folosești match pentru clasificări.
    // Concepte folosite: vec, loops, match.
    // Capcană frecventă: indexare manuală inutilă în loc de iterație.
}

fn mini_test_final_capitol_3() {
    println!("\n[Cap.3][Mini-test final][Provocare]");
    // Obiectiv: combină colecții + control flow într-o funcție completă.
    // Criterii de succes:
    // 1) creezi vector de valori,
    // 2) folosești match pentru fiecare element,
    // 3) întorci un rezumat final (de exemplu: câte pare/impare).
    // Concepte folosite: Vec, for, match, tuple.
    // Capcană frecventă: logică duplicată în ramurile de match.
    println!("Completează mini-testul final în această funcție.");
}

// Exerciții pentru Capitolul 1
// Template unitar pentru fiecare exercițiu:
// - Dificultate
// - Obiectiv
// - Criterii de succes
// - Concepte folosite
// - Capcană frecventă

// Selector rapid:
// - None    => rulează traseul recomandat
// - Some(n) => rulează doar exercițiul n
const EXERCISE_SELECTOR: Option<u8> = None;

pub fn run() {
    println!("\n=== Exerciții: Capitolul 1 ===");
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
        1 => exercitiu_1_1_salut_personalizat(),
        2 => exercitiu_1_2_comentarii_si_tipuri(),
        3 => exercitiu_1_3_bytes_vs_caractere(),
        4 => exercitiu_1_4_inferenta_si_suffix(),
        5 => exercitiu_1_5_operatii_cu_float(),
        6 => exercitiu_1_6_functie_cu_return_implicit(),
        7 => exercitiu_1_7_bloc_care_returneaza_valoare(),
        8 => exercitiu_1_8_debug_struct(),
        9 => exercitiu_1_9_min_max(),
        10 => exercitiu_1_10_mutabilitate_si_shadowing(),
        11 => mini_test_final_capitol_1(),
        _ => println!("[Capitolul 1] Exercițiul {} nu există.", ex_id),
    }
}

fn exercitiu_1_1_salut_personalizat() {
    println!("\n[Cap.1][Ex.1][Bază] Salut personalizat");
    // Obiectiv: folosește String mutabil și adăugare de caracter.
    // Criterii de succes:
    // 1) Creezi String mutabil.
    // 2) Adaugi '!'.
    // 3) Afișezi rezultatul.
    // Concepte folosite: String, mut, push, println!.
    // Capcană frecventă: confuzie între "!" (șir) și '!' (char).
    let mut nume = String::from("Florin Lațcu");
    nume.push('!');
    println!("Salut, {}", nume);
}

fn exercitiu_1_2_comentarii_si_tipuri() {
    println!("\n[Cap.1][Ex.2][Bază] Comentarii și tipuri");
    // Obiectiv: diferențiază inferența de tip față de tip explicit.
    // Criterii de succes:
    // 1) Ai o variabilă inferată.
    // 2) Ai o variabilă explicit tipată care o copiază.
    // 3) Ai comentarii clare pe linii.
    // Concepte folosite: let, type inference, adnotare de tip.
    // Capcană frecventă: comentarii multe, dar nume de variabile neclare.
    let numar_inferat = 42;
    let numar_explicit: i32 = numar_inferat;
    println!(
        "Număr inferat: {}, Număr explicit: {}",
        numar_inferat, numar_explicit
    );
}

fn exercitiu_1_3_bytes_vs_caractere() {
    println!("\n[Cap.1][Ex.3][Bază] Bytes vs caractere");
    // Obiectiv: compară len() cu chars().count().
    // Criterii de succes:
    // 1) Ai un șir ASCII și unul Unicode.
    // 2) Afișezi bytes.
    // 3) Afișezi numărul de caractere.
    // Concepte folosite: len, chars, count.
    // Capcană frecventă: a crede că len() întoarce caractere.
    let ascii = "Florin!";
    let unicode = "Florin Lațcu!";
    println!(
        "ASCII: {} bytes, {} caractere",
        ascii.len(),
        ascii.chars().count()
    );
    println!(
        "Unicode: {} bytes, {} caractere",
        unicode.len(),
        unicode.chars().count()
    );
}

fn exercitiu_1_4_inferenta_si_suffix() {
    println!("\n[Cap.1][Ex.4][Bază] Inferență și suffix");
    // Obiectiv: declară numere cu inferență, tip explicit și suffix.
    // Criterii de succes:
    // 1) Un număr inferat.
    // 2) Un număr u8 explicit.
    // 3) Un i32 cu underscore pentru lizibilitate.
    // Concepte folosite: suffix numeric, adnotare de tip.
    // Capcană frecventă: combinarea tipurilor fără conversie.
    let numar_inferat = 100;
    let numar_u8: u8 = 255;
    let numar_i32 = 1_000_000i32;
    println!(
        "Număr inferat: {}, Număr u8: {}, Număr i32: {}",
        numar_inferat, numar_u8, numar_i32
    );
}

fn exercitiu_1_5_operatii_cu_float() {
    println!("\n[Cap.1][Ex.5][Bază] Operații cu float");
    // Obiectiv: adună f64 cu f32 și calculează media.
    // Criterii de succes:
    // 1) Conversie explicită f32 -> f64.
    // 2) Suma calculată corect.
    // 3) Media afișată cu 2 zecimale.
    // Concepte folosite: cast, format numeric.
    // Capcană frecventă: operare directă f32 + f64.
    let numar_f64: f64 = 3.14159;
    let numar_f32: f32 = 2.71828;
    let suma = numar_f64 + numar_f32 as f64;
    let media = suma / 2.0;
    println!("Sumă: {:.2}, Medie: {:.2}", suma, media);
}

fn exercitiu_1_6_functie_cu_return_implicit() {
    println!("\n[Cap.1][Ex.6][Bază] Funcție cu return implicit");
    // Obiectiv: funcție care întoarce produsul fără `return`.
    // Criterii de succes:
    // 1) Funcție cu doi parametri i32.
    // 2) Return implicit (fără `;` pe ultima expresie).
    // 3) Afișare rezultat.
    // Concepte folosite: funcție, parametri, expresie finală.
    // Capcană frecventă: `;` la final, funcția întoarce `()`.
    fn produs(a: i32, b: i32) -> i32 {
        a * b
    }

    let rezultat = produs(4, 5);
    println!("Produs: {}", rezultat);
}

fn exercitiu_1_7_bloc_care_returneaza_valoare() {
    println!("\n[Cap.1][Ex.7][Combinat] Bloc care întoarce valoare");
    // Obiectiv: compară bloc cu și fără `;` pe ultima linie.
    // Criterii de succes:
    // 1) Primul bloc întoarce sumă.
    // 2) Al doilea bloc întoarce unit `()`.
    // 3) Diferența e afișată clar.
    // Concepte folosite: block expression, unit type.
    // Capcană frecventă: confuzie între expresie și instrucțiune.
    let suma = {
        let a = 10;
        let b = 20;
        a + b
    };
    println!("Suma din bloc: {}", suma);

    let suma_cu_semicolon = {
        let a = 10;
        let b = 20;
        let _ = a + b;
    };
    println!("Suma cu `;`: {:#?}", suma_cu_semicolon);
}

fn exercitiu_1_8_debug_struct() {
    println!("\n[Cap.1][Ex.8][Combinat] Debug pe structură");
    // Obiectiv: definește structură, derive Debug, afișează `{:?}` și `{:#?}`.
    // Criterii de succes:
    // 1) Structură cu cel puțin 2 câmpuri.
    // 2) `#[derive(Debug)]`.
    // 3) Afișare compactă și pretty.
    // Concepte folosite: struct, derive(Debug), formatări.
    // Capcană frecventă: uitarea derive(Debug).
}

fn exercitiu_1_9_min_max() {
    println!("\n[Cap.1][Ex.9][Combinat] Limite numerice");
    // Obiectiv: afișează MIN/MAX pentru tipuri întregi.
    // Criterii de succes:
    // 1) i8/u8, i16/u16.
    // 2) Bonus i32.
    // 3) Format clar al rezultatelor.
    // Concepte folosite: associated consts (MIN/MAX).
    // Capcană frecventă: folosirea tipului greșit la validări.
}

fn exercitiu_1_10_mutabilitate_si_shadowing() {
    println!("\n[Cap.1][Ex.10][Mini-provocare] Mutabilitate + shadowing");
    // Obiectiv: demonstrează diferența dintre `mut` și `shadowing`.
    // Criterii de succes:
    // 1) Variabilă mutabilă schimbată corect.
    // 2) Variabilă shadowed din i32 în f64.
    // 3) Etapele afișate în ordine.
    // Concepte folosite: mut, let shadowing.
    // Capcană frecventă: confundarea mutării de tip cu mutabilitatea.
}

fn mini_test_final_capitol_1() {
    println!("\n[Cap.1][Mini-test final][Provocare]");
    // Obiectiv: combină string, inferență, float, block expression și shadowing.
    // Criterii de succes:
    // 1) Construiești un mesaj final cu numele tău și un scor calculat.
    // 2) Folosești cel puțin un block care întoarce valoare.
    // 3) Folosești o variabilă shadowed într-un pas intermediar.
    // Concepte folosite: String, block expression, shadowing, cast.
    // Capcană frecventă: amestec de tipuri fără conversie.
    println!("Completează mini-testul final în această funcție.");
}

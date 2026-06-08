// Exerciții pentru Capitolul 2
// Template unitar pentru fiecare exercițiu:
// - Dificultate
// - Obiectiv
// - Criterii de succes
// - Concepte folosite
// - Capcană frecventă

const EXERCISE_SELECTOR: Option<u8> = None;

pub fn run() {
    println!("\n=== Exerciții: Capitolul 2 ===");
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
        1 => exercitiu_2_1_stack_heap_referinte(),
        2 => exercitiu_2_2_string_vs_str(),
        3 => exercitiu_2_3_const_static(),
        4 => exercitiu_2_4_return_owned(),
        5 => exercitiu_2_5_mutable_reference(),
        6 => exercitiu_2_6_shadowing_si_referinte(),
        7 => exercitiu_2_7_ownership_in_functii(),
        8 => exercitiu_2_8_copy_vs_clone(),
        9 => exercitiu_2_9_initializare_intarziata(),
        10 => exercitiu_2_10_formatari_avansate(),
        11 => mini_test_final_capitol_2(),
        _ => println!("[Capitolul 2] Exercițiul {} nu există.", ex_id),
    }
}

fn exercitiu_2_1_stack_heap_referinte() {
    println!("\n[Cap.2][Ex.1][Bază] Stack, heap, referințe");
    // Obiectiv: compară `String` și `&str` prin dimensiuni și afișare.
    // Criterii de succes: 1) afișezi size_of::<String>(), 2) size_of_val pe &str, 3) explici diferența.
    // Concepte folosite: stack/heap, String, &str, size_of.
    // Capcană frecventă: confundarea dimensiunii referinței cu dimensiunea conținutului.
}

fn exercitiu_2_2_string_vs_str() {
    println!("\n[Cap.2][Ex.2][Bază] String vs &str");
    // Obiectiv: construiește `String` din `&str` și concatenează cu `format!`.
    // Criterii de succes: 1) conversie corectă, 2) rezultat final afișat, 3) fără clone inutile.
    // Concepte folosite: String::from, to_string, format!.
    // Capcană frecventă: mutarea ownership-ului când nu e necesar.
}

fn exercitiu_2_3_const_static() {
    println!("\n[Cap.2][Ex.3][Bază] const și static");
    // Obiectiv: declară un `const` și un `static` și afișează valorile.
    // Criterii de succes: 1) tipuri explicite, 2) naming clar, 3) utilizare în funcție.
    // Concepte folosite: const, static, tip explicit.
    // Capcană frecventă: presupunerea că inferența funcționează ca la `let`.
}

fn exercitiu_2_4_return_owned() {
    println!("\n[Cap.2][Ex.4][Bază] Return owned");
    // Obiectiv: funcție care întoarce `String` owned.
    // Criterii de succes: 1) semnătură -> String, 2) fără referințe dangling, 3) apel și afișare.
    // Concepte folosite: ownership, return value.
    // Capcană frecventă: întoarcerea `&String` din variabilă locală.
}

fn exercitiu_2_5_mutable_reference() {
    println!("\n[Cap.2][Ex.5][Bază] Mutable reference");
    // Obiectiv: modifică o valoare prin `&mut` și `*`.
    // Criterii de succes: 1) împrumut mutabil, 2) dereference corect, 3) rezultat final vizibil.
    // Concepte folosite: borrow mutabil, dereference.
    // Capcană frecventă: coexistenta invalidă `&` și `&mut` active.
}

fn exercitiu_2_6_shadowing_si_referinte() {
    println!("\n[Cap.2][Ex.6][Bază] Shadowing cu referințe");
    // Obiectiv: arată că shadowing nu invalidează referințe deja create.
    // Criterii de succes: 1) referință validă după shadowing, 2) tip nou pe binding nou.
    // Concepte folosite: shadowing, borrow.
    // Capcană frecventă: presupunerea că referința pointează la binding-ul nou.
}

fn exercitiu_2_7_ownership_in_functii() {
    println!("\n[Cap.2][Ex.7][Combinat] Ownership în funcții");
    // Obiectiv: compară `String`, `&String`, `&mut String` ca parametri.
    // Criterii de succes: 1) demonstrezi move, 2) demonstrezi borrow, 3) demonstrezi mut borrow.
    // Concepte folosite: ownership transfer, borrow read-only, borrow mutabil.
    // Capcană frecventă: reuse după move.
}

fn exercitiu_2_8_copy_vs_clone() {
    println!("\n[Cap.2][Ex.8][Combinat] Copy vs Clone");
    // Obiectiv: compară un tip `Copy` cu `String` + `clone()`.
    // Criterii de succes: 1) tip Copy reutilizat după apel, 2) String demonstrat cu clone, 3) explicație cost.
    // Concepte folosite: Copy, Clone.
    // Capcană frecventă: clone în exces în loc de referință.
}

fn exercitiu_2_9_initializare_intarziata() {
    println!("\n[Cap.2][Ex.9][Combinat] Inițializare întârziată");
    // Obiectiv: declară o variabilă fără valoare și inițializeaz-o într-un block.
    // Criterii de succes: 1) fără folosire înainte de inițializare, 2) afișare după block.
    // Concepte folosite: scope, inițializare ulterioară.
    // Capcană frecventă: presupunerea că mut este obligatoriu.
}

fn exercitiu_2_10_formatari_avansate() {
    println!("\n[Cap.2][Ex.10][Mini-provocare] Formatare avansată");
    // Obiectiv: folosește `println!` cu aliniere, padding, binar/hex/octal și pointer.
    // Criterii de succes: 1) cel puțin 3 stiluri de formatare, 2) output clar, 3) exemplu cu pointer.
    // Concepte folosite: format specifiers.
    // Capcană frecventă: escape-uri greșite în string.
}

fn mini_test_final_capitol_2() {
    println!("\n[Cap.2][Mini-test final][Provocare]");
    // Obiectiv: mini-scenariu cu ownership + borrow + mutable borrow + formatare.
    // Criterii de succes:
    // 1) folosești o funcție cu move, una cu &String și una cu &mut String,
    // 2) eviți clone inutile,
    // 3) afișezi rezultatul final formatat clar.
    // Concepte folosite: ownership, references, mutability.
    // Capcană frecventă: overlap de borrow-uri incompatibile.
    println!("Completează mini-testul final în această funcție.");
}

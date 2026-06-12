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
    for ex_id in [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] {
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
    // Criterii de succes: 1) afișezi size_of::<String>(), 2) size_of::<&str>(), 3) explici diferența.
    // Concepte folosite: stack/heap, String, &str, size_of.
    // Capcană frecventă: confundarea dimensiunii referinței cu dimensiunea conținutului.
    let s = String::from("Hello");
    let s_ref: &str = &s;
    println!("Dimensiune String (tip): {}", std::mem::size_of::<String>());
    println!("Dimensiune &str (tip): {}", std::mem::size_of::<&str>());
    println!("Conținut String: {}", s);
    println!("Conținut &str: {}", s_ref);
    // Explicație: String ocupă 24 bytes (pointer + length + capacity), în timp ce &str ocupă 16 bytes (pointer + length) pe 64-bit. 
    // Ambele sunt referințe, dar String posedă memoria pe heap.
}

fn exercitiu_2_2_string_vs_str() {
    println!("\n[Cap.2][Ex.2][Bază] String vs &str");
    // Obiectiv: construiește `String` din `&str` și concatenează cu `format!`.
    // Criterii de succes: 1) conversie corectă, 2) rezultat final afișat, 3) fără clone inutile.
    // Concepte folosite: String::from, to_string, format!.
    // Capcană frecventă: mutarea ownership-ului când nu e necesar.
    // 1) Construim un &str
    let s_ref: &str = "Salut";
    // 2) Convertim &str în String
    let s_string = s_ref.to_string();
    // 3) Concatenăm folosind format!
    let rezultat = format!("{} lume!", s_string);
    println!("Rezultat final: {}", rezultat);
}

fn exercitiu_2_3_const_static() {
    println!("\n[Cap.2][Ex.3][Bază] const și static");
    // Obiectiv: declară un `const` și un `static` și afișează valorile.
    // Criterii de succes: 1) tipuri explicite, 2) naming clar, 3) utilizare în funcție.
    // Concepte folosite: const, static, tip explicit.
    // Capcană frecventă: presupunerea că inferența funcționează ca la `let`.
    const PI: f64 = 3.14159;
    static GREETING: &str = "Bună ziua!";
    println!("Constanta PI: {}", PI);
    println!("Static GREETING: {}", GREETING);
}

fn exercitiu_2_4_return_owned() {
    println!("\n[Cap.2][Ex.4][Bază] Return owned");
    // Obiectiv: funcție care întoarce `String` owned.
    // Criterii de succes: 1) semnătură -> String, 2) fără referințe dangling, 3) apel și afișare.
    // Concepte folosite: ownership, return value.
    // Capcană frecventă: întoarcerea `&String` din variabilă locală.
    let result = create_owned_string();
    println!("String returnat: {}", result);
}
fn create_owned_string() -> String {
    let s = String::from("Acesta este un String owned returnat.");
    s // ownership-ul lui s este transferat la caller
}

fn exercitiu_2_5_mutable_reference() {
    println!("\n[Cap.2][Ex.5][Bază] Mutable reference");
    // Obiectiv: modifică o valoare prin `&mut` și `*`.
    // Criterii de succes: 1) împrumut mutabil, 2) dereference corect, 3) rezultat final vizibil.
    // Concepte folosite: borrow mutabil, dereference.
    // Capcană frecventă: coexistenta invalidă `&` și `&mut` active.
    let mut numar = 10;
    println!("Înainte de modificare: {}", numar);

    {
        let referinta_mutabila = &mut numar;
        *referinta_mutabila += 15;
        println!("În interiorul împrumutului mutabil: {}", referinta_mutabila);
    }

    println!("După modificare: {}", numar);
}

fn exercitiu_2_6_shadowing_si_referinte() {
    println!("\n[Cap.2][Ex.6][Bază] Shadowing cu referințe");
    // Obiectiv: arată că shadowing nu invalidează referințe deja create.
    // Criterii de succes: 1) referință validă după shadowing, 2) tip nou pe binding nou.
    // Concepte folosite: shadowing, borrow.
    // Capcană frecventă: presupunerea că referința pointează la binding-ul nou.
    let valoare = String::from("București");
    let valoare_ref = &valoare;

    let valoare = 2026;

    println!("Referința veche rămâne validă: {}", valoare_ref);
    println!("Binding-ul nou (alt tip): {}", valoare);
}

fn exercitiu_2_7_ownership_in_functii() {
    println!("\n[Cap.2][Ex.7][Combinat] Ownership în funcții");
    // Obiectiv: compară `String`, `&String`, `&mut String` ca parametri.
    // Criterii de succes: 1) demonstrezi move, 2) demonstrezi borrow, 3) demonstrezi mut borrow.
    // Concepte folosite: ownership transfer, borrow read-only, borrow mutabil.
    // Capcană frecventă: reuse după move.
    let text_initial = String::from("Ownership în Rust");

    citeste_text(&text_initial);

    let mut text_editabil = text_initial;
    adauga_eticheta(&mut text_editabil);
    citeste_text(&text_editabil);

    consuma_text(text_editabil);
}

fn exercitiu_2_8_copy_vs_clone() {
    println!("\n[Cap.2][Ex.8][Combinat] Copy vs Clone");
    // Obiectiv: compară un tip `Copy` cu `String` + `clone()`.
    // Criterii de succes: 1) tip Copy reutilizat după apel, 2) String demonstrat cu clone, 3) explicație cost.
    // Concepte folosite: Copy, Clone.
    // Capcană frecventă: clone în exces în loc de referință.
    let numar_copy = 8;
    afiseaza_numar_copy(numar_copy);
    println!("Tip Copy încă valid după apel: {}", numar_copy);

    let text_original = String::from("Acesta este textul original");
    let text_clonat = text_original.clone();

    consuma_text(text_original);
    println!("Clone păstrat pentru reutilizare: {}", text_clonat);

    let lungime = lungime_text(&text_clonat);
    println!("Alternativă mai ieftină: referință, lungime = {}", lungime);
}

fn exercitiu_2_9_initializare_intarziata() {
    println!("\n[Cap.2][Ex.9][Combinat] Inițializare întârziată");
    // Obiectiv: declară o variabilă fără valoare și inițializeaz-o într-un block.
    // Criterii de succes: 1) fără folosire înainte de inițializare, 2) afișare după block.
    // Concepte folosite: scope, inițializare ulterioară.
    // Capcană frecventă: presupunerea că mut este obligatoriu.
    let scor_final: i32;

    {
        let scor_baza = 50;
        let bonus = 7;
        scor_final = scor_baza + bonus;
        println!("În bloc: scor calculat = {}", scor_final);
    }

    println!("După bloc: scor final = {}", scor_final);
}

fn exercitiu_2_10_formatari_avansate() {
    println!("\n[Cap.2][Ex.10][Mini-provocare] Formatare avansată");
    // Obiectiv: folosește `println!` cu aliniere, padding, binar/hex/octal și pointer.
    // Criterii de succes: 1) cel puțin 3 stiluri de formatare, 2) output clar, 3) exemplu cu pointer.
    // Concepte folosite: format specifiers.
    // Capcană frecventă: escape-uri greșite în string.
    let titlu = "Rust";
    let numar = 42;
    let ref_numar = &numar;

    println!("Aliniere dreapta: |{:>10}|", titlu);
    println!("Padding cu zero: {:08}", numar);
    println!("Binar: {:b}, Hex: {:X}, Octal: {:o}", numar, numar, numar);
    println!("Pointer către număr: {:p}", ref_numar);
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
    let titlu_initial = String::from("rezumat capitol 2");

    citeste_text(&titlu_initial);

    let mut titlu_editabil = titlu_initial;
    adauga_eticheta(&mut titlu_editabil);

    let lungime = lungime_text(&titlu_editabil);
    let titlu_final = finalizeaza_titlu(titlu_editabil);

    println!("Rezultat final:");
    println!("- {:<12}: {}", "Titlu", titlu_final);
    println!("- {:<12}: {}", "Lungime", lungime);
}

fn citeste_text(text: &String) {
    println!("[citire] '{}' are {} caractere", text, text.chars().count());
}

fn adauga_eticheta(text: &mut String) {
    text.push_str(" [verificat]");
    println!("[modificare] text actualizat: {}", text);
}

fn consuma_text(text: String) {
    println!("[move] text consumat: {}", text);
}

fn afiseaza_numar_copy(numar: i32) {
    println!("[copy] valoare primită: {}", numar);
}

fn lungime_text(text: &String) -> usize {
    text.chars().count()
}

fn finalizeaza_titlu(mut titlu: String) -> String {
    titlu.push_str(" ✅");
    titlu
}

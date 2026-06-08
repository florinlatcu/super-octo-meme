# Notițe Capitolul 2

## Sumar Capitolul 2

Capitolul 2, `Memory, variables, and ownership`, este fundația practică pentru felul în care Rust gestionează datele în siguranță.
Parcurgând secțiunile 2.1–2.10, ideea centrală devine clară: ownership este regula implicită, iar references sunt modul controlat de a împrumuta date fără transfer de ownership.
Capitolul acoperă diferența `stack` vs `heap`, tipurile `String` și `&str`, folosirea `const`/`static`, regulile pentru `&` și `&mut`, shadowing în context de references, tipurile `Copy`, variabilele neinițializate și formatarea avansată cu `println!`.
Mesajul-cheie este că Rust preferă reguli stricte la compilare ca să evite bug-uri de memorie la rulare.
Concluzia este că, după capitolul 2, ownership nu mai pare „magie”, ci un model logic și previzibil.

Concepte-cheie:
- Ownership implicit + împrumut cu references (`&T`, `&mut T`).
- `String` este owned; `&str` este borrowed slice.
- Tipurile `Copy` folosesc copiere implicită, nu move.
- `println!` are capabilități extinse de formatare (escape-uri, raw strings, bytes, pointere, aliniere/padding).

Capcană frecventă:
- Folosirea inutilă a `.clone()` sau încercarea de a combina borrow-uri incompatibile (`&` și `&mut`) pe aceeași valoare în același timp.

Exemplu minim:
`let mut s = String::from("Austria"); add_hungary_by_mut_ref(&mut s); println!("{}", s);`

Scor înțelegere: 10/10

## 2.1

1. Secțiunea introduce ideea că `stack` și `heap` sunt două locuri diferite pentru memorie, iar `stack` este de obicei mai rapid.
2. Pentru tipuri simple și cu dimensiune cunoscută la compile time, Rust poate pune valorile pe `stack`.
3. Pentru date cu dimensiune variabilă, cum este `String`, valoarea reală stă pe `heap`, iar pe `stack` apare doar un `pointer` sau o referință.
4. `&str` este prezentat ca un `string slice`, adică o referință la textul respectiv, în timp ce `String` este un tip `owned`.
5. Concluzia practică este că `references`, `String`, `&str` și ideea de `Sized` sunt baza pentru restul capitolului despre memorie și ownership.

Concepte-cheie:
- `stack` este rapid și folosește memorie cu dimensiune cunoscută.
- `heap` este folosit pentru date cu dimensiune variabilă.
- `&` creează o `reference`, iar `String` deține datele.

Capcană frecventă:
- Să încerci să tratezi un `str` ca și cum ar avea dimensiune fixă; în realitate, ai nevoie de `&str`.

Exemplu minim:
`let value = 15; let reference = &value; let text = String::from("Hello");`

Scor înțelegere: 10/10

## 2.2

1. Secțiunea explică cele două tipuri principale de string: `String` și `&str`.
2. `&str` este un `string slice` (pointer + length), adică o vedere peste date, fără ownership.
3. `String` este tip `owned`, cu date pe `heap`, și este mai ușor de modificat (grow/shrink/mutate).
4. Atât `String`, cât și `&str` folosesc UTF-8, deci pot conține text Unicode (inclusiv nume internaționale și emoji).
5. Concluzia practică: `str` simplu nu poate fi folosit direct ca variabilă locală; în practică lucrezi cu `&str` sau `String`.

Concepte-cheie:
- `&str` = `slice` peste text, fără ownership.
- `String` = tip `owned` cu date pe `heap`.
- `size_of` vs `size_of_val` te ajută să vezi diferența dintre tip fix (`Sized`) și valoare dinamică.

Capcană frecventă:
- Să scrii `let x: str = "...";` în loc de `let x: &str = "...";`.

Exemplu minim:
`let a: &str = "Hello"; let b = String::from(a); let c = format!("{}!", b);`

Scor înțelegere: 10/10

## 2.3

1. Secțiunea introduce două moduri de a declara valori fără `let`: `const` și `static`.
2. Pentru ambele, trebuie să specifici explicit `type`-ul (nu există type inference aici).
3. `const` este pentru valori fixe evaluate la compile time, iar `static` este similar, dar are fixed memory location.
4. Valorile globale (`const`/`static`) se declară de obicei cu nume în ALL CAPS și pot fi accesate din orice funcție.
5. Concluzia practică: `const` și `static` sunt bune pentru date globale stabile, dar nu înlocuiesc variabilele obișnuite cu `let`.

Concepte-cheie:
- `const` = valoare constantă cu `type` explicit.
- `static` = valoare globală cu fixed location.
- Valorile globale nu sunt dropped ca variabilele locale.

Capcană frecventă:
- Să încerci să pui în `const`/`static` ceva care depinde de runtime sau necesită heap allocation la compile time.

Exemplu minim:
`const NUMBER_OF_MONTHS: u32 = 12; static SEASONS: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];`

Scor înțelegere: 10/10

## 2.4

1. Secțiunea aprofundează references și arată că poți avea mai multe immutable references către aceeași valoare.
2. Un `&String` doar împrumută datele; ownership rămâne la valoarea originală.
3. Exemplul-cheie arată că nu poți returna o referință către o variabilă locală creată într-o funcție.
4. Motivul este `lifetime`: variabila locală este dropped la finalul block-ului, iar referința ar deveni invalidă.
5. Concluzia practică: când datele trebuie să iasă din funcție, de obicei returnezi valoarea owned (ex: `String`), nu referință la local.

Concepte-cheie:
- `&T` = borrow, nu transfer de ownership.
- Mai multe immutable references sunt permise.
- Nu returnezi referință la date care mor la finalul funcției.

Capcană frecventă:
- Să încerci `fn f() -> &String` când `String` este creat în interiorul acelei funcții.

Exemplu minim:
`fn return_owned_country() -> String { String::from("Austria") }`

Scor înțelegere: 10/10

## 2.5

### 2.5 Mutable references

1. Secțiunea introduce `mutable references`, adică `&mut`, care permit modificarea valorii împrumutate.
2. Când lucrezi cu o referință, folosești dereferencing cu `*` ca să ajungi la valoarea din spatele referinței.
3. Un mod simplu de memorare: `&` face referencing, iar `*` face dereferencing (opusul lui `&`).
4. Exemplul principal arată `*num_ref += 10`, unde modificarea se face prin `&mut`.
5. Concluzia practică: `&mut` este puternic, dar vine cu reguli stricte ca să prevină memory bugs.

### 2.5.1 Rust’s reference rules

Regula 1 (`immutable references`):
- poți avea oricâte `&T` către aceeași valoare (1, 3, 1000…)
- este sigur pentru că toate doar citesc

Regula 2 (`mutable references`):
- poți avea un singur `&mut T` activ la un moment dat
- nu poți combina un `&mut T` activ cu `&T` active pe aceeași valoare

Intuiția: dacă cineva modifică datele, ceilalți cititori nu trebuie să citească simultan ceva care se schimbă.

### 2.5.2 Situation 1: Only one mutable reference

Această situație este validă:
- ai o valoare mutabilă
- creezi un singur `&mut`
- modifici prin referință

Este safe pentru că există un singur „editor” al datelor în acel moment.

### 2.5.3 Situation 2: Only immutable references

Această situație este tot validă:
- poți avea multe `&` în paralel
- toate doar citesc

Este safe pentru că nu există write concurrent în timp ce se citește.

### 2.5.4 Situation 3: The problem situation

Situația invalidă este:
- există deja un `&` activ (reader)
- încerci să creezi `&mut` (writer) pe aceeași valoare

Compilatorul blochează asta (`cannot borrow as mutable because it is also borrowed as immutable`), tocmai ca să prevină comportament neașteptat.

Detaliu important:
- compilatorul modern înțelege mai bine `lifetime`-urile locale (`non-lexical lifetimes`)
- dacă `&mut` nu mai este folosit, poate permite ulterior un `&` în același block, atâta timp cât nu se suprapun efectiv

Concepte-cheie:
- `&mut` + `*` pentru modificare prin referință.
- Multe `&` sunt OK; un singur `&mut` activ este OK.
- `&` și `&mut` active simultan pe aceeași valoare nu sunt OK.

Capcană frecventă:
- Să crezi că ordinea liniilor este suficientă; de fapt contează unde sunt încă active borrow-urile.

Exemplu minim:
`let mut n = 8; let r = &mut n; *r += 10; let read = &n; println!("{}", read);`

Scor înțelegere: 10/10

## 2.6

1. Secțiunea revine la `shadowing` și îl pune în contextul references/ownership.
2. Ideea centrală: `shadowing` nu distruge vechiul value; doar ascunde binding-ul vechi cu unul nou.
3. Dacă ai deja o referință la valoarea veche, acea referință rămâne validă chiar după shadowing.
4. În exemplul din carte, `country_ref` continuă să pointeze la `"Austria"`, deși numele `country` este shadowed cu `8`.
5. Concluzia practică: shadowing schimbă ce înseamnă un nume în scope, nu mută automat referințele deja create.

Concepte-cheie:
- `shadowing` creează un nou binding cu același nume.
- References rămân legate de valoarea originală la care au fost create.
- Același nume poate reprezenta tipuri diferite după shadowing.

Capcană frecventă:
- Să presupui că, după `let country = 8;`, o referință mai veche la `country` ar indica `8` (nu indică).

Exemplu minim:
`let country = String::from("Austria"); let country_ref = &country; let country = 8; println!("{country_ref} {country}");`

Scor înțelegere: 10/10

## 2.7

1. Secțiunea explică cum ownership-ul interacționează cu argumentele funcției.
2. Dacă o funcție primește `String`, ea preia ownership (move), iar valoarea nu mai poate fi folosită după apel.
3. Dacă primește `&String`, funcția doar împrumută datele și poți apela de mai multe ori fără să pierzi ownership-ul.
4. Dacă primește `&mut String`, funcția poate modifica textul fără să devină owner.
5. Concluzia practică: alegi `String` pentru transfer de ownership, `&String` pentru read-only borrow, `&mut String` pentru mutable borrow.

Concepte-cheie:
- `fn f(x: String)` => move în funcție.
- `fn f(x: &String)` => read-only borrow.
- `fn f(x: &mut String)` => mutable borrow.

Capcană frecventă:
- Să chemi de două ori o funcție care primește `String`, folosind aceeași variabilă (a doua folosire dă „use of moved value”).

Exemplu minim:
`let mut country = String::from("Austria"); add_hungary_by_mut_ref(&mut country); println!("{}", country);`

Scor înțelegere: 10/10

## 2.8

1. Secțiunea introduce `Copy types`: tipuri simple, pe `stack`, care se copiază automat când le dai ca argument.
2. Pentru aceste tipuri (ex: `i32`, `bool`, `char`, `f64`), nu te lovești de „move” la fiecare apel.
3. `String` nu implementează `Copy`; când îl dai by value, ownership se mută.
4. Dacă ai nevoie de două valori similare pentru `String`, poți folosi `.clone()`, dar costă memorie.
5. Concluzia practică: dacă nu vrei transfer de ownership, cea mai bună alegere este de obicei o referință immutable (`&T`).

Concepte-cheie:
- `Copy` = copiere implicită, ieftină, pentru tipuri triviale.
- `Clone` = copiere explicită (`.clone()`), utilă dar potențial costisitoare.
- `String` este `Clone`, nu `Copy`.

Capcană frecventă:
- Să folosești `.clone()` în bucle fără motiv, când o referință (`&String`) ar fi suficientă și mai eficientă.

Exemplu minim:
`let country = String::from("Kiribati"); prints_country_owned_2_8(country.clone()); prints_country_owned_2_8(country);`

Scor înțelegere: 10/10

## 2.9

1. Secțiunea introduce variabilele neinițializate: au nume, dar încă nu au primit valoare.
2. În Rust poți declara `let x: i32;`, însă nu poți folosi variabila până nu o inițializezi.
3. Modelul util este: declari variabila în exterior, apoi îi dai valoare dintr-un block intern.
4. Valoarea rămâne validă după block dacă owner-ul (variabila declarată în exterior) o deține.
5. Concluzia practică: uneori e clar și elegant să separi „declarația” de „inițializare”, fără `mut` dacă setezi o singură dată.

Concepte-cheie:
- `uninitialized` = variabilă declarată, dar fără valoare încă.
- Compilerul blochează folosirea înainte de inițializare.
- Nu ai nevoie de `mut` dacă valoarea este atribuită o singură dată.

Capcană frecventă:
- Să încerci să afișezi o variabilă neinițializată; codul nu compilează.

Exemplu minim:
`let my_number; { let calculation_result = 57; my_number = calculation_result; } println!("{my_number}");`

Scor înțelegere: 10/10

## 2.10

1. Secțiunea extinde folosirea `print!`/`println!` cu exemple de formatare mai avansată.
2. `\n` și `\t` controlează liniile și tab-urile, iar `\\` permite afișarea caracterelor escape literal.
3. `raw strings` (`r#"..."#`, `r##"..."##`) simplifică texte cu multe ghilimele/backslash-uri.
4. Poți afișa bytes (`b"..."`, `br##"..."##`), coduri Unicode, adrese de pointer (`{:p}`), dar și baze numerice (`{:b}`, `{:x}`, `{:o}`).
5. Concluzia practică: `println!` este mult mai puternic decât pare și te ajută să controlezi clar output-ul.

Concepte-cheie:
- Escape-uri: `\n`, `\t`, `\\`.
- `raw strings` pentru texte complexe.
- Placeholders cu index/nume și formatare (`padding`, aliniere, lățime).

Capcană frecventă:
- Indentarea accidentală în string-uri multi-line adaugă spații în output.

Exemplu minim:
`println!("Binary: {:b}, hex: {:x}, octal: {:o}", 555, 555, 555);`

Scor înțelegere: 10/10

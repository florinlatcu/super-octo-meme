# Notițe carte Rust 
# Learn Rust in a Month of Lunches

## Sumar Capitolul 1

Capitolul 1, `Some basics`, este o introducere ușoară și variată în Rust.
Cartea spune încă de la început că, în Rust, chiar și exemplele simple pun accent pe `bits`, `bytes` și pe felul în care `type`-urile sunt tratate cu grijă.
Ideea centrală este strictness-ul `compiler`-ului: dacă nu e mulțumit, programul nu rulează, dar îți oferă și `hints` și `suggestions` ca să repari mai repede problema.
Pe parcurs, capitolul acoperă `Introducing Rust`, `comments`, `primitive types`, `type inference`, `Hello, World! and printing`, `declaring variables and code blocks`, `Display` și `Debug`, `smallest and largest numbers`, `mutability` și `shadowing`.
Concluzia mea este că primul capitol te obișnuiește cu felul de a gândi din Rust: mai strict la început, dar foarte util pentru a înțelege mai bine codul și pentru a scrie programe mai sigure.

Concepte-cheie:
- `main` este punctul de intrare al programului.
- Capitolul urmărește o ordine clară: introducere, comments, types, `Hello, World!`, blocks, printing, limits, mutability și shadowing.
- `compiler`-ul oferă feedback util și te ajută să corectezi problemele din timp.

Capcană frecventă:
- Să amesteci concepte care seamănă, dar sunt diferite: `mut` vs `shadowing`, `Display` vs `Debug`, `bytes` vs `characters`, `f32` vs `f64`.

Exemplu minim:
`fn main() { println!("Hello, world!"); }`

Scor înțelegere: 10/10

## 1.1

1. Secțiunea introduce primul contact practic cu Rust.
2. Ideea principală este fluxul simplu: scriu cod, compilez și rulez.
3. Programul începe din `fn main()`, care este punctul de intrare.
4. Exemplul arată folosirea unui `String` mutabil și afișarea lui în consolă.
5. Concluzia este că 1.1 te familiarizează cu baza și cu mediul de lucru.

Concepte-cheie:
- `fn main()` este punctul de intrare.
- `String` se poate modifica dacă variabila este `mut`.
- Compilatorul oferă sugestii utile când apare o eroare.

Capcană frecventă:
- Confuzia dintre `"!"` (șir) și `'!'` (caracter `char`) la `push`.

Exemplu minim:
`let mut my_name = "Dave".to_string(); my_name.push('!'); println!("{}", my_name);`

Scor înțelegere: 10/10

## 1.2

1. Secțiunea explică structura de bază a unui program Rust.
2. Execuția începe în `fn main()`, iar codul este scris între acolade.
3. `println!` este folosit pentru a afișa text în consolă.
4. Instrucțiunile se termină, de obicei, cu `;`, ceea ce face codul mai clar.
5. Concluzia este că 1.2 te ajută să înțelegi forma minimă a unui program Rust.

Concepte-cheie:
- `//` pentru comentarii pe o linie.
- `/* ... */` pentru comentarii pe mai multe linii sau inline.
- Comentariile sunt pentru oameni, compilatorul le ignoră.

Capcană frecventă:
- Comentariile sunt utile, dar nu trebuie să înlocuiască denumiri bune de variabile.

Exemplu minim:
`let some_number = 100; // comentariu scurt`

Scor înțelegere: 10/10

## 1.3

1. Secțiunea introduce tipurile primitive: numere întregi, `char` și șiruri (`&str`/`String`).
2. Diferența-cheie este între tipuri semnate (`i*`) și nesemnate (`u*`), plus mărimea în biți.
3. `char` reprezintă un singur caracter Unicode, iar conversiile cu `as` trebuie făcute atent.
4. Pentru șiruri, `.len()` întoarce numărul de bytes, nu neapărat numărul de caractere.
5. Concluzia practică: pentru număr de caractere folosești `.chars().count()`, iar pentru bytes folosești `.len()`.

Concepte-cheie:
- `i*` = întregi semnați, `u*` = întregi nesemnați.
- `char` este un caracter Unicode (intern are dimensiune fixă).
- Conversiile explicite se fac cu `as`.

Capcană frecventă:
- `.len()` la string dă bytes, nu numărul de caractere vizibile.

Exemplu minim:
`println!("{} / {}", "안녕!".len(), "안녕!".chars().count());`

Scor înțelegere: 10/10

## 1.4

1. Secțiunea explică type inference: Rust ghicește de multe ori tipul corect fără să-l scrii explicit.
2. Pentru numere întregi, tipul implicit este de regulă `i32`, dacă nu specifici altceva.
3. Când vrei control clar, poți declara tipul cu `:` (ex: `let x: u8 = 10`) sau cu sufix (ex: `10u8`).
4. `_` în numere este doar pentru lizibilitate și este ignorat de compilator.
5. Concluzia practică: lasă Rust să infereze când e simplu, dar declară explicit tipul când ai nevoie de precizie.

Concepte-cheie:
- Rust inferează tipul când contextul e clar.
- Implicit, întregii fără tip explicit tind spre `i32`.
- Poți forța tipul cu `:` sau sufix (`10u8`, `100_i32`).

Capcană frecventă:
- Amestecul de tipuri diferite poate duce la erori de tip (mismatched types).

Exemplu minim:
`let a: u8 = 10; let b = 10u8; let c = 100_000_i32;`

Scor înțelegere: 10/10

## 1.5

1. Secțiunea introduce float-urile, adică numerele cu zecimale: `f32` și `f64`.
2. Un număr cu punct (`5.0`, `5.`) nu mai este integer, ci float.
3. Implicit, Rust tinde să folosească `f64` când nu specifici alt tip.
4. Rust nu permite direct operații între `f32` și `f64` fără conversie.
5. Concluzia practică: menține aceleași tipuri la calcule sau convertește explicit când ai nevoie.

Concepte-cheie:
- `f32` și `f64` au precizie/memorie diferită.
- Eroarea „mismatched types” apare când aduni tipuri diferite.
- Conversia explicită se poate face cu `as` (ex: `my_f32 as f64`).

Capcană frecventă:
- Amestecarea `f32` cu `f64` în aceeași expresie fără cast.

Exemplu minim:
`let a: f64 = 5.0; let b: f32 = 8.5; let c = a + b as f64;`

Scor înțelegere: 10/10

## 1.6

1. Secțiunea introduce exemplul clasic `Hello, world!` și explică afișarea cu `println!`.
2. Acoladele `{}` din `println!` pot afișa valori introduse direct sau returnate de funcții.
3. Funcțiile pot primi arguments și pot întoarce values cu `->`.
4. În Rust, ultima expression fără `;` este returnată implicit din funcție.
5. Concluzia practică: `println!`, argumentele funcției și valorile returnate sunt baza pentru secțiunile următoare.

Concepte-cheie:
- `println!` este macro-ul standard pentru output.
- `fn name(...) -> type` definește o funcție care întoarce o value.
- Lipsa lui `;` pe ultima expression permite implicit return.

Capcană frecventă:
- Dacă pui `;` la finalul expresiei returnate, funcția nu mai întoarce expected value.

Exemplu minim:
`fn give_number() -> i32 { 8 } println!("{}", give_number());`

Scor înțelegere: 10/10

## 1.7

1. Secțiunea introduce variable declaration cu `let` și folosirea de `code blocks` (`{}`).
2. O variabilă trăiește doar în interiorul block-ului în care este declarată (`scope`/`lifetime`).
3. `println!` permite atât forma cu argument după virgulă, cât și captura directă în șablon (`{my_number}`).
4. Un `code block` poate returna o value dacă ultima expression nu are `;`.
5. Concluzia practică: controlezi `lifetime` și rezultatul block-urilor prin poziția codului și `semicolon`.

Concepte-cheie:
- `let` face variable declaration.
- `scope` este delimitat de `{}`.
- Fără `;` la ultima expression, block-ul returnează o value.

Capcană frecventă:
- Dacă adaugi `;` la finalul expresiei din block, primești `()` în locul expected value.

Exemplu minim:
`let x = { let y = 8; y + 9 };`

Scor înțelegere: 10/10

## 1.8

1. Secțiunea explică diferența dintre `Display` print (`{}`) și `Debug` print (`{:?}`).
2. Unele tipuri nu implementează `Display`, dar pot fi afișate cu `Debug`.
3. `Debug pretty` (`{:#?}`) afișează datele pe linii separate, mai ușor de citit.
4. `print!` nu adaugă `new line`, în timp ce `println!` adaugă automat `new line`.
5. Concluzia practică: alegi formatter-ul (`Display`/`Debug`) în funcție de tip și scopul output-ului.

Concepte-cheie:
- `{}` folosește `Display` formatter.
- `{:?}` și `{:#?}` folosesc `Debug` formatter.
- `print!` vs `println!` diferă prin `new line` behavior.

Capcană frecventă:
- Încercarea de a afișa cu `{}` un tip care nu implementează `Display` produce compiler error.

Exemplu minim:
`let doesnt_print = (); println!("{:?}", doesnt_print);`

Scor înțelegere: 10/10

## 1.9

1. Secțiunea arată cum afli limitele numerice folosind `MIN` și `MAX` pentru fiecare numeric type.
2. Valorile se accesează ca `associated consts` prin `::` (ex: `i32::MIN`, `u64::MAX`).
3. Exemplele acoperă mai multe integer types: `i8/u8`, `i16/u16`, `i32/u32`, `i64/u64`, `i128/u128`.
4. `MIN` și `MAX` sunt scrise cu majuscule deoarece sunt `const values`.
5. Concluzia practică: când ai nevoie de limite sigure pentru validări, folosești direct `Type::MIN` și `Type::MAX`.

Concepte-cheie:
- `associated const` accesat cu `Type::CONST`.
- `signed` vs `unsigned` numeric ranges.
- `MIN`/`MAX` oferă limitele exacte pentru type-ul ales.

Capcană frecventă:
- Folosirea unui range greșit pentru type-ul variabilei (ex: presupui range de `i32`, dar variabila e `u8`).

Exemplu minim:
`println!("{} {}", i32::MIN, i32::MAX);`

Scor înțelegere: _/10

## 1.10

1. Secțiunea arată că variabilele declarate cu `let` sunt `immutable` dacă nu adaugi `mut`.
2. Cu `mut`, poți schimba valoarea variabilei, dar nu și `type`-ul ei.
3. Dacă încerci să pui o valoare de alt `type`, primești un `mismatched types` error.
4. `mut` este despre changing values, nu despre changing types.
5. Concluzia practică: folosești `mut` când ai nevoie de o valoare care se modifică, dar păstrezi același `type`.

Concepte-cheie:
- `let` fără `mut` => `immutable`.
- `mut` permite changing the value.
- `type` rămâne același chiar dacă variabila este `mut`.

Capcană frecventă:
- Să crezi că `mut` îți permite să schimbi și `type`-ul variabilei.

Exemplu minim:
`let mut my_number = 8; my_number = 10;`

Scor înțelegere: 10/10

## 1.11

1. Secțiunea explică `shadowing`: poți declara o variabilă nouă cu același nume ca una existentă.
2. `shadowing` nu este același lucru cu `mut`; aici creezi un nou `binding`, nu schimbi doar valoarea.
3. Un `shadowed` value poate avea alt `type`, iar vechiul `binding` rămâne ascuns în același `scope`.
4. `shadowing` este util când refolosești un nume și nu vrei să inventezi mereu alt `variable name`.
5. Concluzia practică: folosești `shadowing` pentru pași succesivi pe aceeași idee, mai ales în block-uri mici.

Concepte-cheie:
- `shadowing` creează un nou `binding` cu același nume.
- `scope` decide ce `binding` este vizibil.
- Poți schimba și `type`-ul între `shadowed` bindings.

Capcană frecventă:
- Să confunzi `shadowing` cu `mut`; ele par similare, dar funcționează diferit.

Exemplu minim:
`let x = 8; let x = 9.2; println!("{}", x);`

Scor înțelegere: 10/10

## Sumar Capitolul 2

Capitolul 2, `Memory, variables, and ownership`, este fundația practică pentru felul în care Rust gestionează datele în siguranță.
Parcurgând secțiunile 2.1–2.10, ideea centrală devine clară: ownership este regula implicită, iar references sunt modul controlat de a împrumuta date fără transfer de ownership.
Capitolul acoperă diferența `stack` vs `heap`, tipurile `String` și `&str`, folosirea `const`/`static`, regulile pentru `&` și `&mut`, shadowing în context de references, tipurile `Copy`, variabilele neinițializate și formatarea avansată cu `println!`.
Mesajul-cheie este că Rust preferă reguli stricte la compilare ca să evite bug-uri de memorie la rulare.
Concluzia mea este că, după capitolul 2, ownership nu mai pare „magie”, ci un model logic și previzibil.

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
3. Un mod simplu de memorare din carte: `&` face referencing, iar `*` face dereferencing (opusul lui `&`).
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

Detaliu important din carte:
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

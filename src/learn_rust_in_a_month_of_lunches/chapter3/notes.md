# Notițe Capitolul 3

## Sumar Capitolul 3

Capitolul 3, `More complex types`, leagă două idei foarte importante în Rust: cum grupezi datele și cum controlezi fluxul programului.
În prima parte (3.1), capitolul compară arrays, vectors și tuples, explicând când e mai potrivit fiecare tip.
Mesajul principal este că arrays sunt stricte și rapide (mărime fixă), vectors sunt flexibile (mărime dinamică), iar tuples sunt excelente pentru grupări scurte de tipuri diferite.
În partea a doua (3.2), capitolul trece la `control flow`: `if`, `match`, `loop`, `while`, `for`, plus range-uri exclusive/inclusive și `break` cu valoare.
Concluzia este că, după capitolul 3, codul devine mult mai expresiv: poți descrie clar atât structura datelor, cât și logica de execuție, fără să pierzi siguranța oferită de compilator.

Concepte-cheie:
- Arrays: rapide, dimensiune fixă, un singur tip.
- `Vec<T>`: colecție dinamică, potrivită pentru date care cresc/scad.
- Tuples: grupare de tipuri diferite, destructurare elegantă.
- `match` exhaustiv și loops (`loop`, `while`, `for`) pentru control flow clar.

Capcană frecventă:
- Să alegi tipul de colecție „din reflex” sau să folosești ramuri/loop-uri fără criterii clare de oprire (`break`).

Exemplu minim:
`let result = loop { if ready { break 56; } };`

Scor înțelegere: 10/10

## 3.1

1. Secțiunea introduce `collection types`, adică structuri care țin mai multe valori într-un singur loc.
2. În acest capitol apar trei tipuri principale: arrays, vectors și tuples.
3. Arrays sunt mai simple și mai stricte (mărime fixă), vectors sunt mai flexibile (mărime dinamică), iar tuples pot grupa tipuri diferite.
4. Ideea practică este să alegi colecția în funcție de nevoile de memorie, performanță și flexibilitate.
5. Concluzia secțiunii: 3.1 setează cadrul pentru lucrul cu date compuse în Rust.

Concepte-cheie:
- `collections` = mai multe valori grupate logic.
- Arrays: fixe și rapide în scenarii simple.
- Vectors: redimensionabile, mai ușor de folosit în practică.
- Tuples: utile când vrei tipuri diferite în același grup.

Capcană frecventă:
- Să alegi din reflex `Vec` sau array fără să te uiți la constrângeri (mărime fixă vs dinamică).

Exemplu minim:
`let cities = ["Bucharest", "Cluj"]; // colecție simplă, mărime fixă`

Scor înțelegere: 10/10

## 3.1.1

1. Subcapitolul explică arrays: se declară cu `[]`, conțin elemente de același tip și au mărime fixă.
2. Tipul unui array include și lungimea: forma este `[tip; număr]`, deci `[&str; 2]` și `[&str; 3]` sunt tipuri diferite.
3. Poți crea rapid arrays repetate cu sintaxa `[valoare; număr]` (ex: buffer de bytes `[0u8; 640]`).
4. Arrays permit indexing (`arr[i]`) și slicing (`&arr[2..5]`, `&arr[..]`), iar intervalele pot fi exclusive (`..`) sau inclusive (`..=`).
5. Concluzia practică: arrays sunt excelente când știi dimensiunea dinainte și vrei structură strictă, simplă și eficientă.

Concepte-cheie:
- Array = elemente de același tip + dimensiune fixă.
- Tip explicit: `[T; N]`.
- Indexare cu `[]`, slice cu `&[start..end]`.
- Prefixul `b` transformă textul ASCII într-un array de bytes (`[u8; N]`).

Capcană frecventă:
- Să uiți că lungimea face parte din tip și să tratezi arrays cu lungimi diferite ca fiind același tip.

Exemplu minim:
`let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; let two_to_five = &array_of_ten[2..5];`

Scor înțelegere: 10/10

## 3.1.2

1. Subcapitolul introduce `Vec`, colecția dinamică din Rust, similară cu diferența `String` vs `&str` față de array.
2. Un `Vec` se poate crea cu `Vec::new()` + `.push()` sau mai rapid cu `vec![...]`.
3. Toate elementele dintr-un `Vec` trebuie să aibă același tip, iar tipul poate fi dedus sau declarat explicit (`Vec<String>`).
4. `Vec` are `capacity`; când este depășită, se face realocare (de obicei capacitatea se dublează), deci `Vec::with_capacity(...)` poate fi mai eficient.
5. Concluzia practică: folosești `Vec` când ai nevoie de mărime flexibilă și operații dinamice, iar slicing-ul funcționează similar ca la arrays.

Concepte-cheie:
- `Vec<T>` = colecție dinamică de elemente de tip `T`.
- `vec![]` și `Vec::new()` sunt căi standard de creare.
- `.capacity()` te ajută să urmărești realocările.
- Conversie array -> `Vec` cu `.into()`.

Capcană frecventă:
- Să ignori realocările în bucle mari; dacă știi dimensiunea aproximativă, `with_capacity` reduce costurile.

Exemplu minim:
`let mut v = Vec::with_capacity(8); v.push('a'); println!("{}", v.capacity());`

Scor înțelegere: 10/10

## 3.1.3

1. Subcapitolul introduce tuples, colecții care pot ține tipuri diferite în același grup.
2. Tuple-ul gol `()` este `unit type`; apare frecvent când o funcție nu returnează explicit nimic.
3. Elementele unui tuple se accesează cu notație pe punct (`.0`, `.1`, `.2`), nu cu indexing de array.
4. Destructurarea (`let (a, b, c) = tuple`) permite extragerea rapidă a valorilor în variabile separate.
5. Concluzia practică: tuples sunt utile pentru grupări rapide de date eterogene, mai ales când structura e simplă și locală.

Concepte-cheie:
- Tuple = colecție fixă de valori posibil cu tipuri diferite.
- `()` = unit type.
- Pattern matching la destructurare trebuie să se potrivească exact.

Capcană frecventă:
- Pattern de destructurare greșit (ex: încerci `let (a, b)` pentru un tuple cu 3 elemente).

Exemplu minim:
`let tuple_of_three = ("one", "two", "three"); let (_, b, c) = tuple_of_three;`

Scor înțelegere: 10/10

## 3.2

1. Secțiunea introduce `control flow`, adică modul în care programul alege ce cod să execute în funcție de situație.
2. Ideea centrală este „dacă X este adevărat, execută A; altfel, execută B”.
3. În practică, control flow apare în verificări de condiții, clasificări de valori și repetări.
4. Rust oferă mai multe mecanisme: `if`, `match`, `loop`, `while`, `for`.
5. Concluzia secțiunii: control flow face codul adaptiv, nu doar secvențial.

Concepte-cheie:
- Control flow = decizii + ramuri de execuție.
- Condițiile booleene decid calea de rulare.
- `if` este forma de bază de decizie.

Capcană frecventă:
- Să scrii condiții neclare sau suprapuse, ceea ce duce la logică greu de urmărit.

Exemplu minim:
`let n = 9; if n % 2 == 1 { println!("impar"); }`

Scor înțelegere: 10/10

## 3.2.1

1. Subcapitolul prezintă forma de bază `if {}`: codul se execută doar dacă expresia este `true`.
2. În Rust, compararea folosește `==`, iar `=` rămâne pentru asignare.
3. Poți extinde decizia cu `else if` și `else` pentru mai multe ramuri.
4. Condițiile se pot combina cu operatori logici `&&` (și) și `||` (sau).
5. Concluzia practică: `if`/`else if`/`else` este nucleul deciziilor simple și clare în Rust.

Concepte-cheie:
- `if`, `else if`, `else` pentru ramificare.
- `==` pentru comparație, `=` pentru asignare.
- `&&` și `||` pentru condiții compuse.

Capcană frecventă:
- Confuzia între `==` și `=` în condiții.

Exemplu minim:
`if my_number % 2 == 1 && my_number > 0 { println!("pozitiv impar"); }`

Scor înțelegere: 10/10

## 3.2.2

1. Subcapitolul introduce `match`, o alternativă foarte clară la lanțuri lungi de `if`/`else if`.
2. `match` trebuie să fie exhaustiv: toate cazurile posibile trebuie acoperite (de obicei cu `_` pentru „restul”).
3. Fiecare braț (`arm`) folosește `=>`, iar brațele sunt separate prin virgulă.
4. `match` poate produce direct o valoare și poate folosi pattern-uri mai complexe (tuple, intervale, `@`, guards).
5. Concluzia practică: `match` este excelent pentru clasificări clare și robuste, mai ales când ai multe cazuri.

Concepte-cheie:
- Exhaustivitate (`_` wildcard când nu enumeri toate valorile).
- `match` pe tuple și `match guard` (`if` după pattern).
- Brațele din `match` trebuie să întoarcă același tip când rezultatul este atribuit.

Capcană frecventă:
- Să uiți un caz (fără `_`) sau să întorci tipuri diferite în brațe.

Exemplu minim:
`let second_number = match my_number { 0 => 0, 5 => 10, _ => 2 };`

Scor înțelegere: 10/10

## 3.2.3

1. Subcapitolul introduce `loops`: `loop`, `while` și `for`, fiecare util pentru un tip diferit de repetare.
2. `loop` rulează până când îi spui explicit `break`; poți avea și loop-uri etichetate (`'first_loop`) pentru nested loops.
3. `while` repetă cât timp o condiție este adevărată, fiind mai simplu când ai un criteriu clar de oprire.
4. `for` iterează natural peste range-uri (`0..3`, `0..=3`) și peste colecții; `_` este util când nu folosești variabila de iterație.
5. Concluzia practică: alegerea corectă a loop-ului face codul mai clar, iar `break` poate returna direct o valoare utilă.

Concepte-cheie:
- `loop` + `break` pentru control manual complet.
- Etichete de loop (`'name`) pentru a ieși din loop-ul dorit.
- `while` pentru condiții booleene verificate la fiecare pas.
- `for` + range-uri pentru iterații concise și lizibile.

Capcană frecventă:
- Să folosești `loop` fără condiție de ieșire sau să spargi loop-ul greșit într-un nested loop.

Exemplu minim:
`let my_number = loop { counter += 1; if counter % 53 == 3 { break counter; } };`

Scor înțelegere: 10/10

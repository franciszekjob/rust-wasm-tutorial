# WASM + Rust tutorial

# Spis treści

1. [Czym jest WebAssembly?](#czym-jest-webassembly)
2. [Jak działa WebAssembly?](#jak-działa-webassembly)
3. [Gdzie jest używane WebAssembly?](#gdzie-jest-używane-webassembly)
4. [Zalety i wady WebAssembly](#zalety-i-wady-webassembly)

5. [Jak użyć Rusta z WASM'em - setup projektu](#jak-użyć-rusta-z-wasmem---setup-projektu)
6. [Napisanie prostego przykładu](#napisanie-prostego-przykładu)
7. [Komunikacja Rust <-> JavaScript](#komunikacja-rust--javascript)
8. [Pokazanie finalnego rezultatu tworzonego przykładu](#pokazanie-finalnego-rezultatu-tworzonego-przykładu)
9. [Podsumowanie](#podsumowanie)

## 1. Czym jest WebAssembly?
<img src="./img/wasm.png" width="100px"/><br/>
WASM to nowoczesny format binarny zaprojektowany do wykonywania w przeglądarkach internetowych o wysokiej wydajności. Umożliwia uruchamianie kodu napisane w różnych językach programowania na stronach internetowych, zapewniając prędkość porównywalną z natywnymi aplikacjami.

Garść informacji:
- pracę nad WASM rozpoczęto w 2015 roku
- w grudniu 2019 roku został zaakceptowany jako standard przez W3C
- nazwa nawiązuje do Assemblera, z racji niskopoziomowego kodu
-  został czwartym językiem natywnie obsługiwanym w przeglądarkach internetowych, dołączając do HTML, JavaScript, oraz CSS

## 2. Jak działa WebAssembly?
<img src="./img/wasm_scheme.png"/>
<br/>
<br/>

Proces kompilacji WASM zaczyna się od języka źródłowego, takiego jak C++, Rust lub innego, który jest kompatybilny z WASM.

Kompilacja do WASM: Kod źródłowy jest kompilowany do formatu WebAssembly przez dedykowany kompilator. Na przykład dla C++ używany jest `Emscripten`, a dla Rust - `wasm-pack`.
<br/>

<img src="./img/wasm-pack.png" width="100px"/>
<br/>
<br/>

Generowanie pliku `.wasm`: Kompilator generuje plik `.wasm`, który jest binarny i gotowy do użycia na stronach internetowych.

Załadowanie pliku .wasm na stronie internetowej: Plik jest ładowany do przeglądarki za pomocą JavaScriptu, który następnie może wywoływać z niego funkcje.


W ten sposób WebAssembly pozwala na wykorzystanie kodu z różnych języków programowania na stronach internetowych i zapewnia dużą wydajność w porównaniu z tradycyjnym JavaScriptem.

## 5. Setup projektu

### 5.1 Potrzebne narzędzia

Przed rozpoczęciem tworzenia projektu WebAssembly w Rust-cie, musimy pobrać odpowiednie narzędzia.

#### `Rust`
Z oczywistych względów.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### `wasm-pack`
`wasm-pack` to narzędzie służące m.in. do kompilowania kodu Rust-a do WebAssembly.

```
cargo install wasm-pack
```

### 5.2 Tworzenie projektu
Na początku stwórzmy nową bibliotekę Rust-a o nazwia tutorial.
```
cargo new --lib tutorial
```
W katologu, którym wywołaliśmy te komendę stworzy się taka struktura
```
|--tutorial
   |--Cargo.toml
   |--src
      |--lib.rs
```
W pliku `lib.rs` będziemy pisać kod Rust-a, który będzie kompilowany do **WASM**.

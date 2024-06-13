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

Załadowanie pliku `.wasm` na stronie internetowej: Plik jest ładowany do przeglądarki za pomocą JavaScriptu, który następnie może wywoływać z niego funkcje.

Przykład ładowania pliku `.wasm` w Javascripcie:
```js
(async () => {
  const response = await fetch('fibonacci.wasm');
  const buffer = await response.arrayBuffer();
  const module = new WebAssembly.Module(buffer);
  const instance = new WebAssembly.Instance(module);
  const result = instance.exports.fibonacci(42);
  console.log(result);
})();
```


W ten sposób WASM pozwala na wykorzystanie kodu z różnych języków programowania na stronach internetowych i zapewnia dużą wydajność w porównaniu z tradycyjnym JavaScriptem.
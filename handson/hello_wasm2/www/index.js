async function run() {
    // Aşağıdaki nesne WASM tarafında kullanılabilir
    const logObject = {
        console: {
            log: (param)=> {
                console.log(param+" için alan hesaplanacak.");
            }
        }
    }

    const response = await fetch("calc.wasm");
    const buffer = await response.arrayBuffer();
    const wasm = await WebAssembly.instantiate(buffer,logObject);

    const calcFunc = wasm.instance.exports.calc;
    const result = calcFunc(2);
    console.log(result);
}

run();
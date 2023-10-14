import("../pkg/index.js")
.then(wasm => {
    document.getElementById("cal").textContent = wasm.addrust(1, 2);
})
.catch(console.error);

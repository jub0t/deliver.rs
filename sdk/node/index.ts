import Cdn from "./cdn.rs"

async function main() {
    const cdn = new Cdn("http://127.0.0.1:3434")
    const diagnostics = await cdn.getDiagnostics();
    console.log(diagnostics)
} main();
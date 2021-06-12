Deno.core.ops();
const _newline = new Uint8Array([10]);
function print(value) {
  Deno.core.dispatchByName('op_print', Deno.core.encode(value.toString()), _newline);
}

async function main() {
    Deno.core.print(`running sum`);
}
  
main();
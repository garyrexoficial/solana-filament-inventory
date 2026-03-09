// client.ts
// Script de ejemplo para interactuar con el contrato inventario_filamentos

console.log("Mi dirección:", pg.wallet.publicKey.toString());
const balance = await pg.connection.getBalance(pg.wallet.publicKey);
console.log(`Mi balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);

// 1. Derivar PDA del inventario
const [inventarioPda] = await web3.PublicKey.findProgramAddress(
  [Buffer.from("inventario"), pg.wallet.publicKey.toBuffer()],
  pg.program.programId
);

// 2. Crear inventario
await pg.program.methods
  .crearInventario("Inventario Maker")
  .accounts({
    inventario: inventarioPda,
    owner: pg.wallet.publicKey,
    systemProgram: web3.SystemProgram.programId,
  })
  .rpc();
console.log("Inventario creado en:", inventarioPda.toString());

// 3. Agregar filamento
await pg.program.methods
  .agregarFilamento(
    "PLA",       // tipo
    "Rojo",      // color
    1000,        // peso en gramos
    "Prusa",     // marca
    190,         // temp_min
    220,         // temp_max
    1.75,        // diam_nominal
    0.02,        // diam_tol
    5,           // stock
    450          // precio en MXN
  )
  .accounts({
    inventario: inventarioPda,
    owner: pg.wallet.publicKey,
  })
  .rpc();
console.log("Filamento agregado al inventario");

// 4. Consultar inventario
const inventario = await pg.program.account.inventarioFilamentos.fetch(inventarioPda);
console.log("Inventario actual:", inventario);

describe("Inventario Filamentos", () => {
  it("crear inventario y agregar filamento", async () => {
    // PDA inventario
    const [inventarioPda] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("inventario"), pg.wallet.publicKey.toBuffer()],
      pg.program.programId
    );

    // Crear inventario
    await pg.program.methods
      .crearInventario("Inventario Maker")
      .accounts({
        inventario: inventarioPda,
        owner: pg.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    // Agregar filamento
    await pg.program.methods
      .agregarFilamento(
        "PLA",
        "Rojo",
        1000,
        "Prusa",
        190,
        220,
        1.75,
        0.02,
        5,
        450
      )
      .accounts({
        inventario: inventarioPda,
        owner: pg.wallet.publicKey,
      })
      .rpc();

    // Fetch inventario
    const inventario = await pg.program.account.inventarioFilamentos.fetch(inventarioPda);
    console.log("Inventario:", inventario);
  });
});

use anchor_lang::prelude::*;
// ID del Solana Program, se llenará automáticamente al compilar con "anchor build"
declare_id!("AuCfD6ZgMH4uXnFE7M4hw6f8BPAjLP33pB1kvoy4xYgL");

#[program] // Macro que convierte el código Rust en un programa Solana Anchor
pub mod inventario_filamentos {
    use super::*;

    //////////////////////////// Instrucción: crear_inventario /////////////////////////////////////
    /*
    Crea una cuenta PDA donde se almacenará el inventario de filamentos de impresión 3D.
    Esta cuenta contendrá el struct InventarioFilamentos.

    Parámetros:
        - nombre: nombre descriptivo del inventario
    */
    pub fn crear_inventario(ctx: Context<NuevoInventario>, nombre: String) -> Result<()> {
        let owner_id = ctx.accounts.owner.key();
        msg!("Owner id: {}", owner_id); // Log para verificar

        let filamentos: Vec<Filamento> = Vec::new(); // Vector vacío inicialmente

        ctx.accounts.inventario.set_inner(InventarioFilamentos {
            owner: owner_id,
            nombre,
            filamentos,
        });

        Ok(()) // Transacción exitosa
    }

    //////////////////////////// Instrucción: agregar_filamento /////////////////////////////////////
    /*
    Agrega un filamento al vector dentro del inventario.

    Parámetros:
        - tipo_filamento: ej. PLA, ABS, PETG (String)
        - color: color del filamento (String)
        - peso: peso por rollo (f32, gramos o kg)
        - marca: fabricante (String)
        - temp_min: temperatura mínima de impresión (u16, °C)
        - temp_max: temperatura máxima de impresión (u16, °C)
        - diam_nominal: diámetro nominal (f32, mm)
        - diam_tol: tolerancia del diámetro (f32, mm)
        - stock: cantidad de rollos disponibles (u16)
        - precio_pesos: precio en pesos mexicanos (u64)
    */
    pub fn agregar_filamento(
        ctx: Context<ModificarInventario>,
        tipo_filamento: String,
        color: String,
        peso: f32,
        marca: String,
        temp_min: u16,
        temp_max: u16,
        diam_nominal: f32,
        diam_tol: f32,
        stock: u16,
        precio_pesos: u64,
    ) -> Result<()> {
        // Solo el owner puede modificar el inventario
        require!(
            ctx.accounts.inventario.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let filamento = Filamento {
            tipo_filamento,
            color,
            peso,
            marca,
            temp_min,
            temp_max,
            diam_nominal,
            diam_tol,
            stock,
            precio_pesos,
            owner: ctx.accounts.owner.key(),
        };

        ctx.accounts.inventario.filamentos.push(filamento);

        Ok(())
    }

    //////////////////////////// Instrucción: eliminar_filamento /////////////////////////////////////
    /*
    Elimina un filamento del vector por tipo y color (identificador único aproximado).

    Parámetros:
        - tipo_filamento: String
        - color: String
    */
    pub fn eliminar_filamento(ctx: Context<ModificarInventario>, tipo_filamento: String, color: String) -> Result<()> {
        require!(
            ctx.accounts.inventario.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let filamentos = &mut ctx.accounts.inventario.filamentos;

        for i in 0..filamentos.len() {
            if filamentos[i].tipo_filamento == tipo_filamento && filamentos[i].color == color {
                filamentos.remove(i);
                msg!("Filamento {} de color {} eliminado!", tipo_filamento, color);
                return Ok(());
            }
        }

        Err(Errores::FilamentoNoExiste.into())
    }

    //////////////////////////// Instrucción: ver_filamentos /////////////////////////////////////
    /*
    Muestra en el log todos los filamentos registrados.

    Parámetros:
        - ninguno
    */
    pub fn ver_filamentos(ctx: Context<ModificarInventario>) -> Result<()> {
        require!(
            ctx.accounts.inventario.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("Lista actual de filamentos: {:#?}", ctx.accounts.inventario.filamentos);

        Ok(())
    }

    //////////////////////////// Instrucción: actualizar_filamento /////////////////////////////////////
    /*
    Actualiza atributos seleccionados de un filamento identificado por tipo y color.

    Parámetros:
        - tipo_filamento: String
        - color: String
        - nuevo_peso: Option<f32>
        - nuevo_stock: Option<u16>
        - nuevo_precio_pesos: Option<u64>
    */
    pub fn actualizar_filamento(
        ctx: Context<ModificarInventario>,
        tipo_filamento: String,
        color: String,
        nuevo_peso: Option<f32>,
        nuevo_stock: Option<u16>,
        nuevo_precio_pesos: Option<u64>,
    ) -> Result<()> {
        require!(
            ctx.accounts.inventario.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let filamentos = &mut ctx.accounts.inventario.filamentos;

        for i in 0..filamentos.len() {
            if filamentos[i].tipo_filamento == tipo_filamento && filamentos[i].color == color {
                if let Some(peso) = nuevo_peso {
                    filamentos[i].peso = peso;
                }
                if let Some(stock) = nuevo_stock {
                    filamentos[i].stock = stock;
                }
                if let Some(precio) = nuevo_precio_pesos {
                    filamentos[i].precio_pesos = precio;
                }

                msg!(
                    "Filamento {} (color {}) actualizado: peso={}, stock={}, precio={}",
                    tipo_filamento,
                    color,
                    filamentos[i].peso,
                    filamentos[i].stock,
                    filamentos[i].precio_pesos
                );
                return Ok(());
            }
        }

        Err(Errores::FilamentoNoExiste.into())
    }
}

// Enum para errores personalizados con mensajes claros
#[error_code]
pub enum Errores {
    #[msg("Error: no eres el propietario del inventario")]
    NoEresElOwner,
    #[msg("Error: el filamento con las características especificadas no existe")]
    FilamentoNoExiste,
}

// Cuenta principal que almacena el inventario completo 
#[account]
#[derive(InitSpace)] // Calcula automáticamente el espacio requerido
pub struct InventarioFilamentos {
    pub owner: Pubkey, // Propietario (wallet) del inventario

    #[max_len(60)]
    pub nombre: String, // Nombre representativo del inventario

    #[max_len(50)]
    pub filamentos: Vec<Filamento>, // Vector de filamentos registrados (limitar a 50 para evitar overflow)
}

// Struct para cada filamento 3D con atributos detallados
#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    Clone,
    InitSpace,
    PartialEq,
    Debug
)]
pub struct Filamento {
    #[max_len(20)]
    pub tipo_filamento: String, // Ej: PLA, ABS, PETG

    #[max_len(20)]
    pub color: String,

    pub peso: f32, // peso del rollo (gramos o kg)

    #[max_len(30)]
    pub marca: String, // marca fabricante

    pub temp_min: u16, // temperatura mínima °C

    pub temp_max: u16, // temperatura máxima °C

    pub diam_nominal: f32, // diámetro nominal (mm)

    pub diam_tol: f32, // tolerancia del diámetro (mm)

    pub stock: u16, // cantidad de rollos disponibles

    pub precio_pesos: u64, // precio en pesos mexicanos

    pub owner: Pubkey, // cuenta Solana dueña del filamento
}

// Contexto para la creación de inventario, donde se crea la PDA principal
#[derive(Accounts)]
pub struct NuevoInventario<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = InventarioFilamentos::INIT_SPACE + 8,
        seeds = [b"inventario", owner.key().as_ref()],
        bump
    )]
    pub inventario: Account<'info, InventarioFilamentos>,

    pub system_program: Program<'info, System>,
}

// Contexto para operaciones de modificación (agregar, eliminar, actualizar, ver filamentos)
#[derive(Accounts)]
pub struct ModificarInventario<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub inventario: Account<'info, InventarioFilamentos>,
}

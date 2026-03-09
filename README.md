# solana-filament-inventory
Inventario de Filamentos 3D en Solana
Programa en Rust + Anchor para gestionar un inventario de filamentos de impresión 3D en la blockchain de Solana (Devnet).
Permite registrar, consultar, actualizar y eliminar filamentos con atributos técnicos y económicos.

Características
CRUD completo sobre filamentos de impresión 3D.

Uso de Program Derived Addresses (PDA) para almacenar inventarios.

Validación de permisos: solo el owner puede modificar su inventario.

Atributos de cada filamento:

Tipo (PLA, ABS, PETG, etc.)

Color

Peso del rollo (gramos o kg)

Marca del fabricante

Rango de temperatura de impresión (mínimo y máximo en °C)

Diámetro nominal (1.75 mm) y tolerancia (±0.02 mm)

Stock disponible

Precio en pesos mexicanos

Owner (cuenta Solana asociada)

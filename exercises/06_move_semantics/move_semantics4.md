# Ejercicio move_semantics4

## Descripción

Este es un ejercicio de práctica sobre las reglas de préstamo (borrowing) en Rust.
El código tiene un error intencional que demuestra uno de los conceptos más importantes
del lenguaje: **no se pueden tener múltiples referencias mutables al mismo dato al
mismo tiempo**. El ejercicio pide que solo reordenes las líneas para solucionarlo,
sin agregar, cambiar o eliminar código.

## Desglose paso a paso

El test `move_semantics4()` hace lo siguiente:

- **Línea 1**: Crea un vector vacío mutable llamado `x`
- **Línea 2**: Crea una referencia mutable `y` que apunta a `x`
- **Línea 3**: ❌ **ERROR** - Intenta crear otra referencia mutable `z` que también
   apunta a `x`
- **Línea 4**: Usa `y` para agregar el número 42 al vector
- **Línea 5**: Usa `z` para agregar el número 13 al vector  
- **Línea 6**: Verifica que el vector contenga `[42, 13]`

**El problema**: Rust no permite que existan dos referencias mutables al mismo dato
simultáneamente. Esto previene errores de concurrencia y corrupción de datos.

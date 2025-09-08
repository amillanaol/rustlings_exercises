# Ejercicio move_semantics5

## Descripción

Este script de Rust es un ejercicio de práctica que contiene errores deliberados relacionados con el sistema de ownership (propiedad) y borrowing (préstamo) de referencias. El código incluye dos funciones: una que obtiene el último carácter de un string y otra que imprime un string en mayúsculas. Sin embargo, tiene varios problemas de compilación que necesitan ser corregidos usando únicamente referencias (`&`).

## Desglose paso a paso

### Código original con problemas

```rust
#![allow(clippy::ptr_arg)]

// Shouldn't take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()JJJKj  // ❌ Toma ownership + caracteres basura
}

// Should take ownership
fn string_uppercase(mut data: &String) {  // ❌ Recibe referencia pero debería tomar ownership
    data = data.to_uppercase();  // ❌ No se puede reasignar una referencia
    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();
    
    get_char(data);  // ❌ Transfiere ownership
    string_uppercase(&data);  // ❌ data ya no existe
}
```

### Problemas identificados

**Función `get_char`:**

- Recibe un `String` por valor pero debería recibir una referencia
- Tiene caracteres extraños ("JJJKj") que causan error de sintaxis

**Función `string_uppercase`:**

- Recibe una referencia pero debería tomar ownership
- Intenta reasignar una referencia, lo cual no es válido

**Función `main`:**

- Después de llamar `get_char(data)`, la variable `data` ya no existe


# Ejercicio: functions3

## Descripción

El script proporcionado en Rust define una función `call_me` que imprime mensajes
de llamada en la consola, pero no se ejecuta correctamente debido a un error en la
llamada a la función en el bloque `main`.

## Desglose paso a paso

1. Se define una función `call_me` que acepta un parámetro `num` de tipo `u8`
   (entero sin signo de 8 bits).
2. Dentro de `call_me`, hay un bucle `for` que imprime mensajes "Ring! Call number
   X" donde X va desde 1 hasta `num`.
3. En el bloque `main`, se intenta llamar a la función `call_me` sin proporcionar
   ningún argumento.

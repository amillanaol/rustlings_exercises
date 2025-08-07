# Ejercicio: functions1

## Descripción

El script proporcionado es un programa básico escrito en Rust que llama a una función
llamada `call_me` desde la función `main`. Sin embargo, la función `call_me` aún
no está definida.

## Desglose paso a paso

1. **Definición de la función `main`**: La función `main` es el punto de entrada
   de cualquier programa Rust. En este caso, la función `main` llama a otra función
   llamada `call_me`.
2. **Llamada a `call_me`**: Dentro de `main`, se encuentra la línea `call_me();`,
   que intenta llamar a la función `call_me`. Sin embargo, dado que `call_me` no
   está definida, el programa no se compilará.
3. **Necesidad de definir `call_me`**: Para que el programa se compile y ejecute
   correctamente, es necesario definir la función `call_me`. Dado que se
   especifica que `call_me` no debe tener argumentos ni retornar valor alguno,
   su definición debería ajustarse a este requisito.

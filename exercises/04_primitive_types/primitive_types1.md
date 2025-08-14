# Ejercicio primitive_types1

## Descripción

El script proporcionado es un ejercicio en Rust que trabaja con variables booleanas
y condicionales `if`. El objetivo es imprimir mensajes según el momento del día (mañana
o noche) basado en el valor de una variable booleana.

## Desglose paso a paso

1. Se define una variable booleana `is_morning` con el valor `true`.
2. Se utiliza una sentencia `if` para comprobar el valor de `is_morning`. Si es `true`,
   imprime "Good morning!".
3. Se pide definir otra variable booleana `is_evening` antes de otra sentencia `if`.
4. El valor de `is_evening` debe ser la negación de `is_morning`, es decir, si `is_morning`
   es `true`, `is_evening` debe ser `false`, y viceversa.
5. La segunda sentencia `if` comprueba el valor de `is_evening` y, si es `true`,
   imprime "Good evening!".

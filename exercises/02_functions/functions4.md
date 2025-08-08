# Ejercicio: functions4

## Descripción

El script está diseñado para calcular el precio de venta de un artículo en una tienda
que ofrece descuentos basados en si el precio original es un número par o impar.
Si el precio es par, se descuentan 10 unidades, y si es impar, se descuentan 3 unidades.

## Desglose paso a paso

1. **Definición de la función `is_even`**: Esta función toma un número entero de
64 bits (`i64`) como entrada y devuelve un valor booleano (`bool`). La función determina
si el número es par verificando si el resto del número dividido por 2
es igual a 0.

2. **Definición de la función `sale_price`**: Actualmente, esta función tiene una
firma incompleta y un error en la sintaxis de retorno. Está destinada a calcular
el precio de venta de un artículo basado en su precio original. Si el precio original
es par (devuelto por `is_even`), el precio de venta será el precio original menos
10; de lo contrario, será el precio original menos 3.

3. **Función `main`**: En la función principal, se define un precio original (`original_price`)
de 51 y luego se imprime el precio de venta calculado usando la función `sale_price`.

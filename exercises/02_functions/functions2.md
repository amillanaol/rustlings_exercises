# Ejercicio: functions2

## Descripción

El script proporcionado es un ejercicio en Rust que define una función `call_me`
que imprime mensajes de llamada. La función toma un argumento `num` de tipo desconocido
y utiliza un loop para imprimir mensajes.

## Desglose paso a paso

1. Se define una función `call_me` que toma un argumento `num` sin tipo especificado.
2. La función utiliza un loop `for` para iterar desde `0` hasta `num - 1`.
3. Dentro del loop, se imprime un mensaje "Ring! Call number X", donde X es el número
   de iteración más uno.
4. En la función `main`, se llama a `call_me` con el argumento `3`.

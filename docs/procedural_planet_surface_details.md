# Procedural Planet Surface Details

## Objetivo

Mejorar la identidad visual de los planetas sin depender todavía de texturas externas ni shaders custom.

## Cambios implementados

- Detalles procedurales sobre planetas principales.
- Variación visual para Tierra, Marte, Venus, Mercurio, Urano y Neptuno.
- Bandas procedurales para Júpiter.
- Bandas procedurales para Saturno.
- Rotación visual ligera de detalles superficiales.
- Tests para selección de cuerpos, bandas y rangos de escala.

## Principio visual

La escena sigue siendo realista, pero los cuerpos ya no dependen solamente de esferas de color plano.

## Limitación actual

Los detalles se crean con pequeños marcadores esféricos sobre la superficie.

Esto es suficiente para lectura visual temprana, pero no reemplaza todavía:

- Texturas reales.
- Shaders WGSL.
- Normal maps.
- Materiales atmosféricos.
- Bloom real.

## Próximos pasos

- Mejorar anillos de Saturno con malla anular real.
- Añadir bandas más suaves mediante shader.
- Añadir textura procedural de nubes para la Tierra.
- Añadir bloom/postprocesado.

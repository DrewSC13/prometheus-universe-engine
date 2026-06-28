# Earth Procedural Landmasses

## Objetivo

Mejorar la identidad visual de la Tierra agregando continentes procedurales sobre la esfera base.

## Cambios implementados

- Continentes procedurales generados por cúmulos sobre esfera.
- Materiales diferenciados para zonas verdes y áridas.
- Rotación visual suave de los continentes.
- Integración con la posición visual de la Tierra calculada por catálogo.
- Tests de constantes, direcciones, rangos y cantidad visible.

## Enfoque

La implementación sigue sin usar texturas externas.

Los continentes se representan como marcadores pequeños agrupados, suficientes para que la Tierra sea reconocible a distancia dentro de la escala educativa actual.

## Limitación actual

Los continentes todavía son discretos, no una textura continua.

En una fase posterior deberían migrarse a:

- malla esférica con atributos procedurales;
- textura generada en runtime;
- shader WGSL para océano/tierra/nubes;
- normal maps o mapas de altura opcionales.

## Estado

Refinamiento posterior a `earth_atmosphere_clouds`.

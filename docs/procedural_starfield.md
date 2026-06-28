# Procedural Starfield

## Objetivo

Agregar un fondo espacial procedural para mejorar la sensación visual del Sistema Solar.

## Implementación

El campo de estrellas se genera sin assets externos usando pequeños marcadores esféricos emisivos distribuidos sobre una esfera grande alrededor de la escena.

## Parámetros iniciales

- 900 estrellas.
- Radio del campo: 320 unidades visuales.
- Escala mínima: 0.018.
- Escala máxima: 0.060.

## Decisión técnica

Se usa una distribución basada en ángulo dorado para repartir estrellas de forma estable y uniforme.

## Ventajas

- No requiere texturas externas.
- Es determinista.
- Es liviano.
- Mejora inmediatamente la presentación visual.

## Próximos pasos

- Variar colores por temperatura.
- Añadir brillo por magnitud.
- Añadir skybox real.
- Añadir nebulosas de fondo.

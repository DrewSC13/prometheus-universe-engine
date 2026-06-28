# Refined Procedural Planet Details

## Objetivo

Refinar los detalles visuales procedurales de los planetas para que se vean menos como esferas agregadas y más como textura superficial.

## Cambio visual

Antes, los detalles de superficie podían verse como protuberancias grandes.

Ahora:

- Los detalles son más pequeños.
- Están más pegados a la superficie.
- Hay mayor cantidad de puntos sutiles.
- El efecto visual se aproxima más a textura procedural ligera.
- Se reduce la apariencia de maqueta o planeta cubierto de bolitas.

## Valores ajustados

- PLANET_SURFACE_FEATURE_COUNT: 96.
- PLANET_SURFACE_RADIUS_FACTOR: 1.012.
- PLANET_SURFACE_MIN_SCALE: 0.010.
- PLANET_SURFACE_MAX_SCALE: 0.034.

## Nota técnica

Esto sigue siendo un sistema procedural educativo.

El siguiente paso visual realista será migrar estos detalles hacia materiales, shaders, mapas procedurales o texturas PBR.

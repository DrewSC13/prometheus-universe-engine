# Earth Atmosphere and Clouds

## Objetivo

Mejorar la identidad visual de la Tierra dentro del Sistema Solar realista.

## Cambios implementados

- Atmósfera azul tenue mediante capas transparentes.
- Nubes procedurales mediante marcadores ligeros sobre la superficie.
- Movimiento visual suave de nubes.
- Pulso atmosférico muy sutil.
- Tests para validar constantes, dirección y escala.

## Enfoque

Esta implementación no usa texturas externas.

La atmósfera y las nubes son procedurales y siguen la posición visual de la Tierra calculada desde el catálogo.

## Limitación actual

Las nubes todavía son marcadores discretos.

En una fase posterior deberían migrarse a:

- textura procedural;
- shader WGSL;
- material atmosférico real;
- scattering aproximado;
- texturas opcionales de alta resolución.

## Estado

Implementación inicial posterior al tag v0.2.0-phase1.

# Solar HDR Bloom Real Glow

## Objetivo

Reemplazar el aspecto de discos concéntricos del halo solar por un brillo más realista basado en Bloom, tonemapping y material emissive.

## Cambios

- Cámara 3D compatible con Bloom según API actual de Bevy.
- Bloom natural en cámara cuando la versión de Bevy lo permite por componente.
- Tonemapping TonyMcMapface para mejor respuesta visual en luces intensas.
- Material del Sol con emisión elevada.
- Halo falso de esferas reducido a opacidad casi invisible.

## Razón

El halo por geometría transparente genera discos visibles. El material emissive y Bloom producen una sensación de resplandor más cercana a una fuente luminosa real, evitando depender de campos HDR no disponibles en esta versión de Bevy.

## Estado

Refinamiento visual experimental de Fase 1.

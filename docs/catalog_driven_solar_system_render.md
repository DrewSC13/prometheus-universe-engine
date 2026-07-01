# Catalog Driven Solar System Render

## Objetivo

Migrar el render desde una escena Sol-Tierra-Luna escrita manualmente hacia un render basado en catálogo.

## Cambio principal

El render ahora usa SOLAR_SYSTEM_BODIES como fuente principal para crear:

- Cuerpos visuales.
- Colores.
- Radios visuales.
- Órbitas.
- Etiquetas.

## Cuerpos renderizados

- Sol.
- Mercurio.
- Venus.
- Tierra.
- Luna.
- Marte.
- Júpiter.
- Saturno.
- Urano.
- Neptuno.

## Principio técnico

La simulación mantiene posiciones físicas en metros usando f64.

La visualización usa escala realista para permitir ver el Sistema Solar completo en una escena navegable.

## Estado

Implementado como evolución de SolarSystemRenderPlugin.

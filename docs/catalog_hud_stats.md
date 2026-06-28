# Catalog HUD Stats

## Objetivo

Mostrar información básica del catálogo del Sistema Solar directamente en el HUD.

## Información agregada

- Cuerpos totales.
- Cuerpos raíz.
- Cuerpos orbitando.

## Motivo

La Fase 1 usa SOLAR_SYSTEM_BODIES como fuente central para simulación y render.

El HUD debe reflejar que el sistema ya está dirigido por catálogo y no por una escena Sol-Tierra-Luna escrita manualmente.

## Estado

Implementado en:

engine/src/ui/hud.rs

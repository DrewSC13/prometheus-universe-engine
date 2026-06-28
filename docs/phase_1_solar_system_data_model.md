# Phase 1 Solar System Data Model

## Objetivo

Crear un modelo genérico de datos para cuerpos celestes antes de expandir el render.

## Cuerpos incluidos

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

## Componentes del modelo

- BodyId.
- BodyClass.
- OrbitDefinition.
- CelestialBodyDefinition.
- Catálogo SOLAR_SYSTEM_BODIES.

## Principio técnico

La simulación y el render no deben depender de datos escritos manualmente en sistemas específicos.

La Fase 1 debe mover el motor hacia un catálogo centralizado que después pueda alimentar:

- Simulación.
- Render.
- HUD.
- UI.
- Pipeline de datos.
- Futuras fuentes científicas.

## Estado

Modelo inicial implementado en:

engine/src/simulation/bodies.rs

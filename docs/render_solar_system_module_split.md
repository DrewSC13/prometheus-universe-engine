# Render Solar System Module Split

## Objetivo

Reducir el tamaño de `engine/src/render/solar_system.rs` sin cambiar comportamiento de runtime.

## Primer corte seguro

Se movieron los módulos de tests a archivos separados bajo:

`engine/src/render/solar_system/`

## Módulos extraídos

- `tests.rs`
- `real_solar_halo_glow_tests.rs`
- `earth_atmosphere_cloud_tests.rs`
- `earth_landmass_tests.rs`
- `saturn_ring_mesh_tests.rs`

## Razón

`solar_system.rs` superó las 2000 líneas. Antes de dividir lógica de producción, se separaron primero los tests porque es el refactor con menor riesgo funcional.

## Estado

No cambia simulación, render ni UI. Solo reorganiza módulos de test.

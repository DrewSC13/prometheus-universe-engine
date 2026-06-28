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


## Segundo corte seguro: Saturno

Se extrajo la lógica visual de Saturno/anillos a:

`engine/src/render/solar_system/saturn.rs`

Incluye:

- constantes de anillos de Saturno;
- componentes visuales de anillos;
- generación de malla anular;
- marcadores de anillos;
- actualización de posición de anillos según el cuerpo padre.

Este corte mantiene la API interna disponible para `solar_system.rs` mediante:

`mod saturn;`
`use self::saturn::*;`


## Tercer corte seguro: Tierra

Se extrajo la lógica visual de Tierra a:

`engine/src/render/solar_system/earth.rs`

Incluye:

- constantes de atmósfera;
- constantes de nubes;
- constantes de landmasses;
- componentes visuales de Tierra;
- spawn/update de atmósfera;
- spawn/update de nubes;
- spawn/update de continentes procedurales;
- helpers usados por tests.

Este corte mantiene disponible la API interna para `solar_system.rs` mediante:

`mod earth;`
`use self::earth::*;`

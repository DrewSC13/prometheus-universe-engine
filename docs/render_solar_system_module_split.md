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


## Cuarto corte seguro: Sol

Se extrajo la lógica visual del Sol a:

`engine/src/render/solar_system/sun.rs`

Incluye:

- constantes de luz solar;
- constantes de superficie solar;
- constantes de corona;
- constantes de halo solar real;
- componentes visuales solares;
- spawn/update de features de superficie solar;
- spawn/update de marcadores de corona;
- glow/halo solar real;
- helpers usados por tests.

Este corte mantiene disponible la API interna para `solar_system.rs` mediante:

`mod sun;`
`use self::sun::*;`


## Quinto corte seguro: Starfield

Se extrajo la lógica visual del campo estelar a:

`engine/src/render/solar_system/starfield.rs`

Incluye:

- constantes de starfield;
- componente visual de estrellas;
- spawn del campo estelar;
- helpers de posición, escala y material.

Este corte mantiene disponible la API interna para `solar_system.rs` mediante:

`mod starfield;`
`use self::starfield::*;`


## Sexto corte seguro: Órbitas

Se extrajo la lógica visual orbital a:

`engine/src/render/solar_system/orbits.rs`

Incluye:

- constantes de escalado orbital;
- componente visual de marcadores orbitales;
- spawn de marcadores orbitales;
- actualización de marcadores orbitales;
- visibilidad orbital;
- helper de radio realista de órbita;
- helpers de cantidad y tamaño de marcadores orbitales.

Este corte mantiene disponible la API interna para `solar_system.rs` mediante:

`mod orbits;`
`use self::orbits::*;`


## Séptimo corte seguro: Labels

Se extrajo la lógica visual de etiquetas a:

`engine/src/render/solar_system/labels.rs`

Incluye:

- recurso `LabelVisibilityMode`;
- componente visual `SolarBodyLabel`;
- controles de teclado para etiquetas;
- creación de labels;
- actualización de labels hacia cámara;
- visibilidad de labels;
- helpers de tamaño, color y offset;
- helper de selección de cuerpos principales.

Este corte mantiene disponible la API interna para `solar_system.rs` mediante:

`mod labels;`
`use self::labels::*;`


## Octavo corte seguro: Superficies planetarias

Se extrajo la lógica visual genérica de superficies planetarias a:

`engine/src/render/solar_system/planet_surface.rs`

Incluye:

- constantes de detalles superficiales;
- constantes de bandas planetarias;
- componentes visuales de detalles y bandas;
- spawn de detalles superficiales;
- actualización de detalles superficiales;
- actualización de bandas planetarias;
- helpers de selección de planetas con detalle;
- helpers de escala, dirección, material y bandas.

Este corte mantiene disponible la API interna para `solar_system.rs` mediante:

`mod planet_surface;`
`use self::planet_surface::{...};`

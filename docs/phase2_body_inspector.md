# Phase 2 Body Inspector

## Objetivo

Iniciar la Fase 2 con una base de interacción segura: selección de cuerpos celestes desde teclado.

## Primer corte

Se crea el módulo:

`engine/src/interaction/selection.rs`

## Recurso principal

`SelectedBody`

Representa el cuerpo actualmente seleccionado:

- `Some(BodyId)`: hay cuerpo seleccionado.
- `None`: no hay selección activa.

## Controles

- `N`: selecciona el siguiente cuerpo del catálogo.
- `P`: selecciona el cuerpo anterior del catálogo.
- `Escape`: limpia la selección.

## Fuente de datos

La selección usa `SOLAR_SYSTEM_BODIES` y `body_definition`, por lo que sigue el catálogo central del sistema solar.

## Estado

Este corte todavía no implementa raycast/picking con mouse.

La siguiente mejora sugerida es mostrar la selección en el HUD y luego añadir resaltado visual del cuerpo seleccionado.

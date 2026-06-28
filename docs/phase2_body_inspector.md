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


## Segundo corte: selección visible en HUD

El HUD ahora muestra el cuerpo seleccionado.

En modo compacto se añade:

`seleccion: <nombre>`

En modo completo se añade un bloque:

- selección activa;
- clase del cuerpo;
- radio físico aproximado;
- masa;
- cuerpo padre orbital.

Controles visibles en HUD:

- `N`: siguiente cuerpo.
- `P`: cuerpo anterior.
- `Escape`: limpiar selección.


## Tercer corte: indicador visual de selección

Se añade un indicador visual transparente para el cuerpo seleccionado.

Comportamiento:

- si hay selección activa, el indicador se posiciona sobre el cuerpo seleccionado;
- si no hay selección, el indicador se oculta;
- el tamaño del indicador escala con el radio visual del cuerpo;
- los cuerpos pequeños conservan un padding mínimo para que el indicador siga siendo visible.

Este corte todavía no implementa picking con mouse.


## Cuarto corte: enfocar cámara en el cuerpo seleccionado

Se añade navegación básica del Body Inspector.

Control nuevo:

- `G`: enfoca la cámara en el cuerpo seleccionado.

Comportamiento:

- si hay cuerpo seleccionado, la cámara se posiciona cerca del cuerpo y mira hacia él;
- si no hay selección activa, no mueve la cámara;
- el sistema conserva un mínimo de distancia para cuerpos pequeños;
- el sistema limita la distancia máxima para cuerpos grandes.

Implementación:

- `engine/src/interaction/focus.rs`
- `BodyFocusPlugin`
- `solar_body_visual_position` como wrapper público seguro para obtener posiciones visuales del render solar.


## Quinto corte: selección con mouse

Se añade selección directa con mouse.

Control nuevo:

- `Click izquierdo`: selecciona el cuerpo solar bajo el cursor.

Implementación:

- `engine/src/interaction/picking.rs`
- `BodyPickingPlugin`
- conversión de cursor a rayo con `Camera::viewport_to_world`;
- intersección aproximada rayo-esfera contra `SolarBodyVisual`;
- selección del cuerpo más cercano intersectado;
- actualización de `SelectedBody`.

Este corte usa picking geométrico manual y no requiere dependencias nuevas.

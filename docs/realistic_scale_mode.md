# Realistic Scale Mode

## Objetivo

Eliminar la escala educativa y usar una escala orbital realista proporcional.

## Modelo

El render convierte metros físicos a unidades de escena usando una relación fija:

- 1 AU = 36 unidades de escena.
- Las posiciones orbitales salen de `body_position_meters`.
- Las órbitas usan `semi_major_axis_meters` directamente.
- La Luna mantiene su distancia orbital real proporcional respecto a la Tierra.

## Nota de navegación

El sistema solar real es muy vacío. Por eso las cámaras generales se alejaron y el enfoque con `G` sigue siendo esencial para inspeccionar cuerpos seleccionados.

## Estado

No existe modo educativo en este corte. La escala orbital publicada es realista y proporcional.

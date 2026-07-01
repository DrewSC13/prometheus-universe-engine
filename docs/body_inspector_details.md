# Body Inspector Details

## Objetivo

Ampliar el inspector de cuerpo seleccionado con datos físicos y orbitales útiles.

## Datos agregados

El HUD de selección ahora incluye:

- distancia real al Sol;
- distancia real al cuerpo padre orbital;
- periodo orbital;
- periodo de rotación;
- inclinación axial;
- clase del cuerpo en español ASCII;
- aviso de escala orbital realista.

## Modelo de distancia

Las distancias se calculan desde `body_position_meters`, usando el modelo orbital circular del catálogo.

## Escala visual

El render mantiene escala orbital realista: las posiciones físicas existen en metros, pero el render mantiene distancias proporcionales para navegación y lectura.

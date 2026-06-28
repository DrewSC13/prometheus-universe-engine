# Floating Origin Runtime

## Objetivo

Mantener la cámara cerca del origen local para evitar jitter visual cuando se navegue a grandes distancias.

## Estrategia

- La posición global vive en GlobalPosition con f64.
- La posición local de render vive en Transform con f32.
- La cámara tiene GlobalPositionComponent.
- Cuando la cámara supera el umbral, el origen se mueve a la posición global de la cámara.
- Los objetos con GlobalPositionComponent se recalculan en coordenadas locales.

## Umbral inicial

10.000 km, equivalente a 10.000.000 metros.

## Estado

Implementado como FloatingOriginRuntimePlugin.

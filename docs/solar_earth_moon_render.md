# Render Sol-Tierra-Luna

## Objetivo

Dibujar los primeros cuerpos astronómicos del motor:

- Sol.
- Tierra.
- Luna.

## Estrategia

La simulación mantiene posiciones físicas en metros usando f64.

El render usa una escala visual comprimida para representar distancias astronómicas dentro de una escena navegable.

## Controles

- W/A/S/D: movimiento horizontal.
- Q/E: bajar/subir.
- Click derecho + mouse: mirar.
- Shift: acelerar.
- Ctrl: reducir velocidad.

## Estado

Implementado como SolarSystemRenderPlugin.

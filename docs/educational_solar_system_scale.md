# Escala educativa Sol-Tierra-Luna

## Objetivo

Hacer que Sol, Tierra y Luna sean visibles al mismo tiempo durante la Fase 0.

## Decisión

Las posiciones físicas reales se mantienen en metros usando f64.

El render usa una escala educativa separada para que los cuerpos puedan verse juntos en una escena navegable.

## Valores iniciales

- Radio visual del Sol: 3.0 unidades.
- Radio visual de la Tierra: 0.65 unidades.
- Radio visual de la Luna: 0.22 unidades.
- Órbita visual de la Tierra: 18.0 unidades.
- Órbita visual de la Luna: 2.4 unidades.

## Tiempo

La simulación inicia con una escala temporal de x50.000 para que el movimiento orbital sea visible.

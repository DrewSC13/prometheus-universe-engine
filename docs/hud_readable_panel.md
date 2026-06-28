# HUD Readable Panel

## Objetivo

Mejorar la legibilidad del HUD cuando cuerpos brillantes, especialmente el Sol, quedan detrás del texto.

## Cambio

El HUD ahora usa un panel oscuro semitransparente mediante `BackgroundColor`.

## Comportamiento

- HUD completo: fondo negro con mayor opacidad.
- HUD compacto: fondo negro más ligero.
- HUD oculto: fondo transparente y texto vacío.

Este corte no modifica simulación, selección, cámara ni render de cuerpos.

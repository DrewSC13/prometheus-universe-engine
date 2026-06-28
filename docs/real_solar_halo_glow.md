# Real Solar Halo Glow

## Objetivo

Reemplazar la percepción de corona basada principalmente en marcadores por un halo solar real más suave.

## Problema anterior

El Sol tenía textura procedural y actividad alrededor, pero el supuesto brillo se percibía como muchas esferas pequeñas alrededor del cuerpo solar.

Eso no se veía como un resplandor natural.

## Solución

Se añade una capa visual compuesta por varias esferas transparentes concéntricas alrededor del Sol.

Cada capa:

- Tiene mayor radio que la anterior.
- Tiene menor opacidad que la anterior.
- Usa material unlit.
- Usa alpha blending.
- Rodea al Sol desde el origen.

También se añade una luz puntual solar para reforzar la sensación de fuente luminosa.

## Resultado esperado

El Sol debe verse con:

- Núcleo brillante.
- Superficie procedural.
- Halo externo suave.
- Corona de partículas más secundaria.
- Fondo espacial negro, no plomo.

## Estado

Implementado como `spawn_real_solar_halo_glow`.

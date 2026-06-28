# Procedural Solar Glow

## Objetivo

Mejorar visualmente el Sol sin depender todavía de texturas externas, shaders custom ni assets descargados.

## Cambios implementados

- Fondo espacial más oscuro.
- Starfield más denso.
- Variación básica de color en estrellas.
- Mayor potencia emissive del Sol.
- Superficie solar procedural mediante marcadores luminosos.
- Corona/halo solar procedural mediante dos capas de marcadores.
- Rotación lenta de la superficie solar y de la corona.

## Decisión técnica

Se usa una aproximación procedural basada en pequeños marcadores esféricos emisivos.

Esta estrategia permite mejorar la lectura visual del Sol antes de introducir:

- Shaders WGSL.
- Texturas solares reales.
- Bloom postprocess.
- Normal maps.
- Ruido procedural en GPU.

## Estado visual esperado

El Sol debe dejar de verse como una esfera amarilla plana y comenzar a verse como un cuerpo luminoso con:

- Variación superficial.
- Puntos calientes.
- Halo cercano.
- Corona exterior.
- Más presencia visual en escena.

## Próximos pasos

- Añadir bloom.
- Crear shader procedural solar.
- Añadir textura de granulación.
- Añadir atmósfera/corona más suave con transparencia.

# Órbitas visuales y etiquetas

## Objetivo

Mejorar la lectura visual de la escena Sol-Tierra-Luna durante la Fase 0.

## Implementación inicial

Las órbitas se representan mediante pequeños marcadores esféricos:

- Órbita de la Tierra alrededor del Sol.
- Órbita de la Luna alrededor de la Tierra.

Esta decisión evita introducir todavía shaders, líneas GPU o geometría avanzada.

## Etiquetas

Se agregan etiquetas 2D para:

- Sol.
- Tierra.
- Luna.

## Nota técnica

Las posiciones físicas siguen siendo calculadas en metros usando f64.

La escena visual usa escala educativa para facilitar comprensión y depuración.

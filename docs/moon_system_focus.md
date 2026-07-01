# Moon System Focus

## Objetivo

Mejorar el enfoque con `G` cuando el cuerpo seleccionado tiene lunas.

## Comportamiento

- Si el cuerpo seleccionado no tiene lunas, el enfoque usa la distancia normal por radio visual.
- Si el cuerpo seleccionado tiene satélites, el enfoque calcula el radio local del sistema de lunas.
- La cámara se aleja lo suficiente para mostrar planeta, órbitas locales y lunas principales.
- El Sol se mantiene como enfoque de cuerpo individual; no usa todos los planetas como si fueran lunas.

## Resultado esperado

- `G` sobre Júpiter muestra Júpiter y las lunas galileanas con mejor contexto.
- `G` sobre Saturno muestra Saturno, anillos y lunas principales.
- `G` sobre Sol mantiene una vista cercana del Sol.

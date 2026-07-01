# Camera View Presets

## Objetivo

Agregar presets de cámara para navegar rápidamente durante la Fase 1.

## Controles

- C: vista general del Sistema Solar realista.
- V: vista lejana / panorámica.
- F: vista interior.

## Comportamiento

Cada preset:

- Reposiciona la cámara.
- Apunta hacia el origen visual del sistema.
- Reinicia la posición global de cámara a GlobalPosition::ZERO.

## Motivo

La escena ya contiene múltiples cuerpos y órbitas, por lo que conviene tener formas rápidas de recuperar una vista útil.

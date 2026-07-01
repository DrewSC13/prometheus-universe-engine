# Axial Body Rotation

## Objetivo

Agregar rotación axial a cuerpos del sistema solar usando periodos de rotación aproximados.

## Alcance del corte

- Cada cuerpo obtiene periodo de rotación en horas.
- Se incluye inclinación axial aproximada.
- El `Transform` del cuerpo visual rota según días desde J2000.
- Los detalles visibles de superficie, nubes, bandas, anillos de Saturno y superficie solar siguen la rotación axial.
- Venus, Urano y Tritón usan periodo negativo para representar rotación retrógrada.

## Escala visual

La posición orbital sigue siendo realista: se conserva la dirección física calculada desde el catálogo, pero la distancia orbital se mantiene proporcional al dato físico para que el sistema sea navegable.

## Luna

La Luna conserva rotación sincrónica aproximada: su periodo de rotación coincide con su periodo orbital medio del catálogo.

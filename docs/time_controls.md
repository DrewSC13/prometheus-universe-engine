# Controles de tiempo

## Objetivo

Permitir manipular SimulationTime desde teclado durante la Fase 0.

## Controles

- Space: pausar o reanudar.
- 1: velocidad x1.
- 2: velocidad x100.
- 3: velocidad x1.000.
- 4: velocidad x10.000.
- 5: velocidad x50.000.
- 6: velocidad x1.000.000.
- B: invertir dirección del tiempo.
- R: reiniciar a J2000 con velocidad x50.000.

## Principio

RealTime sigue separado de SimulationTime.

El frame usa tiempo real del motor.

La simulación astronómica usa SimulationTime.

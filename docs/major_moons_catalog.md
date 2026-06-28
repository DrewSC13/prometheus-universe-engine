# Major Moons Catalog

## Objetivo

Ampliar la Fase 1 agregando lunas principales al catálogo central del Sistema Solar.

## Lunas agregadas

### Júpiter

- Io.
- Europa.
- Ganímedes.
- Calisto.

### Saturno

- Titán.
- Encélado.
- Rea.

### Urano

- Titania.
- Oberón.

### Neptuno

- Tritón.

## Decisión técnica

Las lunas se agregan como CelestialBodyDefinition con BodyClass::NaturalSatellite.

Cada luna define:

- Cuerpo padre.
- Radio orbital medio.
- Periodo orbital.
- Fase inicial.
- Radio físico.
- Masa.
- Radio visual.
- Color base.

## Estado

El render por catálogo puede mostrar estas lunas sin crear sistemas nuevos específicos.

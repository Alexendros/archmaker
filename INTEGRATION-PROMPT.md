# Prompt para integrar sobre el plan ya iniciado

```text
Integra este paquete en el repositorio ArchMaker existente sin reiniciar el trabajo.

Reglas:
1. Compara por ruta y contenido.
2. `reference/v5.1/` debe reemplazarse por los bytes de este paquete y verificarse con SHA256SUMS.
3. Si un documento existente es más completo, conserva su contenido e incorpora solo requisitos, IDs, decisiones o evidencias que falten.
4. Si este paquete contradice una decisión humana ya aceptada, no sobrescribas: registra la contradicción y pide decisión.
5. No marques ADR ni gates como accepted/complete sin evidencia.
6. No inicies código funcional.
7. Produce una tabla por archivo: replace, merge, keep-existing o new; explica por qué.
8. Ejecuta validaciones documentales y de JSON.
9. Convierte todos los P0 ausentes en issues ordenados por dependencia.
10. Devuelve árbol final, diff resumido, riesgos P0, DECISION-REQUIRED y Go/No-Go.
```

# Recolector de vigilancia ACTA

Recolector semanal, mecánico y auditable. No usa un LLM, no resume el contenido y no modifica
el cerebro del proyecto. Compara feeds, APIs, XML o un bloque HTML con el estado anterior y
escribe `vigilancia/AAAA-Www.md`.

## Ejecución local

Requiere Python 3.11 o posterior y no instala dependencias:

```bash
python3 tools/vigilancia/main.py
python3 -m unittest discover -s tools/vigilancia/tests -v
```

La primera ejecución de cada fuente crea su línea base y no anuncia como nuevas todas las
entradas históricas.

## Configuración pendiente

- **A3 agentproto/BoF:** falta el grupo definitivo. VCON y DAWN sí están activos.
- **A6 EUR-Lex:** crear las búsquedas guardadas en My EUR-Lex y guardar un secret de Actions
  llamado `ACTA_EURLEX_FEED_URLS_JSON`, con un array JSON de URLs RSS.
- **A9 EAS/Base:** no se activa hasta fijar red, unidad de coste, ventana semanal, línea base y
  significado exacto de «un orden de magnitud». El adaptador de anclaje no contiene esas
  decisiones económicas.
- **A5 LOTL:** los patrones provisionales deben confirmarse contra los identificadores oficiales
  que publique la Comisión para los nuevos servicios cualificados eIDAS 2.

Los cambios de listas cortas y umbrales se realizan en `config.toml` y deben revisarse en cada
sesión mensual de investigación.

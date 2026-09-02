# Validación de mercado — ¿existe hoy demanda pagada por evidencia verificable por terceros de decisiones automatizadas?

> **Nota de incorporación.** Investigación de señales duras observables (desk research
> multi-fuente) realizada el **31 de agosto de 2026** como respuesta al §9 de la
> `Directiva de construcción`. Es el material de evidencia que motiva la enmienda **E-9**
> (§0). Naturaleza: **research**, no normativo. Las conclusiones estratégicas derivadas de
> él viven en E-9; este documento contiene los hallazgos y sus fuentes.
>
> **Separación hecho / inferencia:** las secciones 1–7 son hallazgos con fuente. La sección
> "Síntesis" contiene inferencias estratégicas del operador y de Claude, marcadas como
> tales. Todas las cifras de tamaño de mercado provienen de analistas con metodologías
> divergentes y deben tratarse como orden de magnitud.

**Fecha de la investigación:** 31 de agosto de 2026
**Pregunta de decisión:** ¿existe hoy demanda real y pagada — no interés teórico — por
evidencia verificable por terceros de decisiones automatizadas, especialmente en banca y
sanidad europeas?

---

## Resumen

- Existe un mercado pagado y en rápido crecimiento para **gobernanza y aseguramiento** de IA,
  pero casi todo ese gasto compra el modelo estándar "logs del operador + atestación /
  procedimiento firmado" (tipo SOC 2 / ISO 42001), **no** evidencia
  verificable-sin-confiar-en-el-operador, que es la propuesta central de ACTA.
- La distinción de ACTA es real y algunos vendedores la comercializan (Guardtime TrueTrail,
  Codenotary/immudb, Hedera Consensus Service), pero es un nicho, no el estándar que los
  compradores exigen ni por el que pagan una prima documentada.
- Los reguladores europeos ya exigen trazabilidad y "audit trail" (EBA loan origination, ECB
  Guide to Internal Models con sección de ML de julio 2025, FDA PCCP) y hay sanciones
  documentadas alrededor de decisiones automatizadas — pero esos requisitos se satisfacen
  hoy con logs convencionales + gobernanza.
- **Veredicto:** hay comprador para "evidencia/aseguramiento de IA" hoy; **no** hay evidencia
  dura de que exista hoy un comprador que pague una prima específicamente por
  trustless/third-party-verifiable. El mercado de cumplimiento del AI Act está en formación
  (logging de alto riesgo aplazado a 2-dic-2027 / 2-ago-2028 por el Reglamento UE 2026/1744).

---

## 1. Competidores cobrando hoy

### 1.1 Plataformas de AI governance / GRC (modelo atestación)

El segmento se consolidó lo bastante como para tener su primer **Gartner Magic Quadrant for
AI Governance Platforms** el 16 de junio de 2026, con Credo AI como "Visionary" y Holistic AI
como "Challenger".

- **Credo AI** (fundada 2020, Palo Alto): suscripción empresarial por cotización, sin precio
  público, en marketplaces de AWS y Microsoft. Estimaciones de terceros: $30K–$150K/año (no
  confirmadas por el vendedor). Producto: AI Registry, Risk Intelligence, Policy Engine,
  "audit-ready evidence". **Su "Audit Trail" es un registro estructurado de GRC — quién hizo
  cambios y cuándo — explícitamente no criptográfico ni tamper-evident.**
- **Monitaur** (seguros y servicios financieros): captura decisiones de modelo en un log
  "firmado criptográficamente"; 33 controles mapeados a NAIC, ISO 42001, NIST AI RMF y EU AI
  Act. Se posiciona como *system of record del propio operador* — assurance vía testing y
  controles, no verificabilidad por terceros.
- **IBM watsonx.governance**, **Holistic AI** (auditoría/red-teaming, sesgos NYC LL144),
  **Fiddler**, **Arthur** (monitorización). Ninguno publica precios; todos por cotización.

**Hallazgo:** ningún líder de mercado vende verificabilidad independiente offline como
propuesta principal.

### 1.2 Logging inmutable / anchoring (algunos SÍ venden verificabilidad)

- **Amazon QLDB**: fin de soporte 31-jul-2025, sin anuncio formal; migración recomendada a
  Aurora PostgreSQL, que pierde la verificabilidad criptográfica. Señal ambivalente: AWS
  retiró un ledger criptográficamente verificable (presumible baja demanda) pero dejó un
  hueco.
- **Guardtime / TrueTrail** (sobre KSI Blockchain, en producción desde 2012): el caso más
  claro de venta de verificabilidad-sin-confianza. Promete audit trails presentables como
  prueba legalmente sólida con prueba independiente de tiempo, integridad y orden;
  posicionamiento explícito de "prueba matemática" frente a "es verdad porque yo lo digo".
  Dirigido expresamente a banca y sanidad. Cliente desplegado: ministerio del interior
  estonio (SMIT), con logs usados como prueba judicial, procesando solo hashes. Financiación
  EU Horizon 2020 (grant 881092). **Catorce años en el mercado y sigue siendo nicho.**
- **Codenotary / immudb**: "la única base de datos inmutable a escala empresarial con
  verificación criptográfica"; menciona explícitamente registrar decisiones y acciones de
  sistemas automatizados o agentes de IA. immudb 1.11 lanzado en mayo de 2026.
- **Hedera Consensus Service**: cada resultado de consenso auditable independientemente.
  Precio público: `ConsensusSubmitMessage` de $0,0001 a $0,0008 USD por mensaje desde enero
  de 2026.

## 2. Sanciones y expedientes

- **SCHUFA (CJEU C-634/21, 7-dic-2023):** el scoring crediticio automatizado constituye
  decisión del art. 22 GDPR cuando juega un papel determinante. Sanciones del art. 22 en el
  nivel superior: hasta €20M o 4% de facturación global.
- **Foodinho/Glovo (Garante italiano):** dos sanciones. €2,6M el 5-jul-2021 (~19.000 riders)
  y €5M el 22-nov-2024 (35.000+ riders), esta última incluyendo sistemas de decisión
  automatizada sin capacidad de los riders de contestar las decisiones. El Garante ordenó
  que operadores entrenados verificasen las decisiones algorítmicas.
- **Toeslagenaffaire (Países Bajos):** ~26.000 familias acusadas erróneamente por un modelo
  de riesgo que usaba nacionalidad; dimisión del gobierno Rutte (ene-2021); multa de €3,7M
  de la Autoriteit Persoonsgegevens.
- **SyRI (Tribunal de La Haya, 2020):** legislación declarada ilegal por violar el art. 8
  CEDH; el tribunal señaló la falta de transparencia del modelo.

**Hallazgo:** los expedientes castigan **discriminación, falta de base legal y opacidad**.
Ninguno ha citado la integridad criptográfica del log como el fallo.

## 3. Licitaciones públicas

No se localizó ninguna licitación que exija explícitamente audit trails criptográficamente o
independientemente verificables para decisiones de IA/automatizadas. Lo que existe:

- requisitos genéricos de "audit trail"/trazabilidad en licitaciones de IA e IT (p. ej.
  framework NHS "Healthcare AI Solutions", notice 043057-2026);
- el AI Act como contexto que clasifica la contratación pública como uso de alto riesgo,
  exigiendo transparencia, trazabilidad y supervisión;
- interés español en DLT para contratación pública garantizando inmutabilidad de la
  información (presentación de política, no tender);
- **STS 1119/2025 (caso BOSCO):** el Tribunal Supremo español reconoció el derecho de acceso
  al código fuente de un algoritmo público, ampliando la "información pública" a los
  elementos técnicos de decisiones automatizadas.

**Limitación metodológica declarada:** las interfaces de TED y UK Find a Tender se renderizan
por JavaScript y no pudieron consultarse a texto completo. La ausencia es "no encontrada", no
"probada". Verificación definitiva requeriría TED Expert Search o los datasets abiertos en
bloque.

## 4. Guías supervisoras

- **EBA Guidelines on Loan Origination and Monitoring (EBA/GL/2020/06**, aplicables desde
  30-jun-2021): exigen infraestructura de datos que permita audit trailing efectivo y
  seguimiento de desviaciones, excepciones y overrides de rating/scoring.
- **ECB Guide to Internal Models (revisado 28-jul-2025):** nueva sección 9 sobre ML aplicable
  a todos los modelos de Pilar 1. Exige explicabilidad, complejidad justificada, registro de
  modelos con control de versiones, y como buena práctica trazabilidad (p. ej. versionado)
  para registrar decisiones y permitir replicación y auditabilidad de outputs. Herramientas
  de explicabilidad evaluadas al menos anualmente.
- **FDA (SaMD con IA/ML):** guía final PCCP (dic-2024, implementación completa ago-2025):
  protocolos de modificación documentados, evaluaciones de impacto, requisitos de
  trazabilidad. >1.350 dispositivos con IA autorizados a principios de 2026.

**Hallazgo:** "trazabilidad" aparece por todas partes; **ningún supervisor exige hoy que un
tercero pueda verificar la integridad del log sin confiar en el operador.**

## 5. Estándares y mercado laboral

- **ISO/IEC 42001:2023** (publicada 18-dic-2023): más de 350 organizaciones certificadas
  globalmente en primavera de 2026 (no hay registro oficial único). AWS primer gran cloud
  (nov-2024); KPMG Australia primera globalmente (oct-2024); GMV España certificada por
  AENOR (abr-2026). **ISO/IEC 42006:2025** acredita a los organismos certificadores.
- **DSIT UK, "Assuring a Responsible Future for AI"** (6-nov-2024): 524 empresas de AI
  assurance (84 especializadas), £1,01bn de valor añadido bruto, 12.572 empleados,
  proyección a £6,53bn en 2035.
- **ISACA AAIA** (mayo-2025): primera credencial específica de auditoría de IA.
- **NYC Local Law 144** (vigor jul-2023): auditorías de sesgo anuales independientes para
  herramientas de empleo automatizadas.
- **Gartner AI TRiSM:** ~$3,1bn en 2025, proyección a $13,8bn en 2030 (CAGR ~35%). Otras
  estimaciones divergen ($2,34bn 2024 → $7,44bn 2030 según Grand View; $6,02bn 2030 según
  SNS Insider).
- **C2PA / Content Credentials:** infraestructura mainstream de procedencia **de contenido**
  (no de decisiones); Pixel 10 firma todas las fotos por defecto.

## 6. La distinción clave

Buscado específicamente: ¿alguien vende o exige "verificabilidad sin confiar en el operador"
frente al estándar actual (logs propios + atestación tipo SOC 2)?

- **Vendedores que la comercializan:** Guardtime, Codenotary/immudb, Hedera, OriginStamp.
- **Prima de precio documentada por la distinción:** ninguna encontrada.
- **Plataformas mainstream de AI governance que la ofrezcan:** ninguna; los analistas la
  describen como un **hueco** del mercado, no como capacidad comprada de forma estándar.
- **Evidencia de que los compradores distingan y paguen más:** no encontrada.

## 7. Economía de agentes

- **Google AP2** (16-sept-2025, 60+ socios incluidos Mastercard, PayPal, Coinbase): Intent /
  Cart / Payment Mandates como **W3C Verifiable Credentials**; registros firmados
  criptográficamente que sirven como prueba verificable de que el usuario autorizó una
  transacción. Pueden alimentarse a ERPs; los reguladores pueden solicitarlos como
  historiales auditables.
- **Coinbase x402** (mayo-2025): revive HTTP 402 para micropagos en stablecoin. Gobernado por
  la x402 Foundation con Google, Visa, AWS, Circle, Anthropic, Vercel. Stripe soporta x402
  como vía de liquidación (preview mid-2026); Cloudflare y AWS con soporte a nivel edge.
- **Visa TAP** y **PayPal Agent Ready** completan el stack.
- **OpenWorker** (Andrew Ng, MIT, 17k⭐): vende "governance is the architecture" y un audit
  trail con procedencia de aprobación por tool call — implementado como registro local
  auto-atestado, sin encadenamiento ni anclaje externo.

**Hallazgo:** el stack resuelve **autorización** y **liquidación**. Los pagos son finales y
sin chargebacks. No hay incumbente para **ejecución probada** ni para evidencia de disputa
entre agentes.

---

## Síntesis — INFERENCIA ESTRATÉGICA (no hallazgo)

*Esta sección contiene el razonamiento del operador y de Claude sobre los hallazgos
anteriores. Es la base de la enmienda E-9 y debe leerse como interpretación, no como dato.*

**Lo que soporta "existe comprador hoy":** mercado de assurance real y creciente; vendedores
cobrando; vendedores de verificabilidad con clientes desplegados; presión regulatoria
concreta; escándalos y multas reales; y un stack de pagos de agentes que genera recibos
verificables de forma nativa.

**Lo que lo contradice para la propuesta específica de ACTA:** el gasto compra atestación, no
trustless-verifiability; ningún tender pide verificabilidad por terceros para IA; ningún
supervisor la exige; el cierre de QLDB sugiere demanda limitada por ledger verificable puro;
el AI Act está aplazado; los líderes de gobernanza tratan la verificabilidad criptográfica
como hueco, no como capacidad comprada. **Y el dato más incómodo: Guardtime lleva desde 2012
vendiendo exactamente esta propuesta a banca y sanidad, y sigue siendo nicho.**

**Explicación estructural propuesta:** el comprador de cumplimiento compra *lo que el
supervisor acepta*, y todos los supervisores aceptan hoy atestación. La demanda de ACTA en
cumplimiento no depende de convencer a responsables de compliance: **depende de que un
supervisor o un estándar armonizado (CEN-CENELEC JTC21, definiendo qué significa "logging" en
el AI Act) exija integridad verificable.** Ese disparador llega en 2027.

**Consecuencia adoptada (E-9):** primer comprador = economía de agentes, donde la
verificabilidad sin confianza no es prima sino prerequisito (no hay operador humano cuya
palabra aceptar; sin chargebacks, la evidencia de ejecución es el único mecanismo de disputa).
Cumplimiento humano = segundo mercado, que madura en 2027 con disparador supervisor.

**Lo que internet no puede responder:** la disposición a pagar real y la elasticidad de precio
por "verificable sin confianza" frente a atestación. No existe ni un dato público que
cuantifique una prima. Esto solo se resuelve con pilotos pagados y conversaciones de compra
directas — de ahí que las conversaciones de validación no desaparezcan, solo cambien de
interlocutor (E-9.8).

## Señales a vigilar (disparadores que cambiarían el diagnóstico)

1. Estándares armonizados CEN-CENELEC JTC21: si definen "logging" del AI Act como integridad
   verificable.
2. El primer caso de ejecución donde el fallo citado sea la **integridad del log** (aún no ha
   ocurrido).
3. Tenders TED/PLACSP que pidan "immutable/verifiable" para sistemas de IA.
4. Un supervisor (EBA/ECB/BaFin/Banco de España) exigiendo verificabilidad por terceros
   explícita, no solo "audit trail".

Cualquiera de los cuatro convierte la hipótesis del mercado de cumplimiento en apuesta con
base observable.

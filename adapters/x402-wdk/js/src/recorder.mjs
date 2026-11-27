// ACTA x402 exchange recorder (implementation candidate, Proposed ADR-019).
//
// Records one x402 v2 exchange exactly as it crossed the wire, on either side of it. It
// keeps header values and body bytes as transmitted and never parses, re-encodes or
// interprets them: turning them into evidence is the Rust mapper's job, and it commits to
// the bytes held here.
//
// No dependencies and no Node built-ins, so the same file loads on Node.js and on Bare.

export const EXCHANGE_SCHEMA = 'acta.x402.exchange.v0'

const H_REQUIRED = 'payment-required'
const H_SIGNATURE = 'payment-signature'
const H_RESPONSE = 'payment-response'

const B64 = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/'

export function bytesToBase64 (bytes) {
  let out = ''
  for (let i = 0; i < bytes.length; i += 3) {
    const a = bytes[i]
    const b = i + 1 < bytes.length ? bytes[i + 1] : 0
    const c = i + 2 < bytes.length ? bytes[i + 2] : 0
    out += B64[a >> 2] + B64[((a & 3) << 4) | (b >> 4)]
    out += i + 1 < bytes.length ? B64[((b & 15) << 2) | (c >> 6)] : '='
    out += i + 2 < bytes.length ? B64[c & 63] : '='
  }
  return out
}

function requireRef (value, name) {
  if (typeof value !== 'string' || value.trim() === '') {
    throw new TypeError(`${name} must be a non-empty string`)
  }
  return value
}

function isoNow (now) {
  return new Date(now()).toISOString()
}

function readHeader (headers, name) {
  if (!headers) return null
  if (typeof headers.get === 'function') return headers.get(name)
  for (const key of Object.keys(headers)) {
    if (key.toLowerCase() === name) {
      const value = headers[key]
      return Array.isArray(value) ? value.join(', ') : String(value)
    }
  }
  return null
}

function requestParts (input, init) {
  const isRequest = typeof input === 'object' && input !== null && 'url' in input
  return {
    method: String((init && init.method) || (isRequest && input.method) || 'GET').toUpperCase(),
    url: isRequest ? input.url : String(input),
    signature:
      readHeader(init && init.headers, H_SIGNATURE) ??
      (isRequest ? readHeader(input.headers, H_SIGNATURE) : null)
  }
}

/**
 * Buyer side. Wrap the fetch that the x402 client wraps:
 *
 *   const recorder = createBuyerRecorder({ recorderRef, counterpartyRef })
 *   const fetchWithPayment = wrapFetchWithPayment(recorder.wrap(fetch), client)
 *
 * A record is produced for each request that carried PAYMENT-SIGNATURE. Requests that never
 * reached that point are not exchanges and produce nothing.
 */
export function createBuyerRecorder ({ recorderRef, counterpartyRef, now = Date.now, onRecord } = {}) {
  requireRef(recorderRef, 'recorderRef')
  requireRef(counterpartyRef, 'counterpartyRef')
  const records = []
  const challenges = new Map()

  function wrap (innerFetch) {
    return async function recordedFetch (input, init) {
      const { method, url, signature } = requestParts(input, init)
      const key = `${method} ${url}`
      const signatureSeenAt = signature === null ? null : isoNow(now)
      const response = await innerFetch(input, init)

      if (signature === null) {
        const required = readHeader(response.headers, H_REQUIRED)
        if (response.status === 402 && required !== null) challenges.set(key, required)
        return response
      }

      const body = new Uint8Array(await response.clone().arrayBuffer())
      const record = {
        schema: EXCHANGE_SCHEMA,
        side: 'buyer',
        recorder_ref: recorderRef,
        counterparty_ref: counterpartyRef,
        request: { method, url },
        payment_required_header: challenges.get(key) ?? null,
        payment_signature_header: signature,
        response_status: response.status,
        payment_response_header: readHeader(response.headers, H_RESPONSE),
        response_body_b64: bytesToBase64(body),
        signature_seen_at: signatureSeenAt,
        response_seen_at: isoNow(now)
      }
      challenges.delete(key)
      records.push(record)
      if (onRecord) onRecord(record)
      return response
    }
  }

  return { wrap, records }
}

/**
 * Seller side. Express/Connect-style middleware, mounted BEFORE the x402 payment middleware
 * so it sees the request as received and the response as sent:
 *
 *   const recorder = createSellerRecorder({ recorderRef, counterpartyRef: (req) => ... })
 *   app.use(recorder.middleware)
 *   app.use(paymentMiddleware(...))
 *
 * The seller does not re-send PAYMENT-REQUIRED on the paid request, so its record carries
 * `payment_required_header: null`. The accepted terms travel inside PAYMENT-SIGNATURE.
 */
export function createSellerRecorder ({ recorderRef, counterpartyRef, now = Date.now, onRecord } = {}) {
  requireRef(recorderRef, 'recorderRef')
  if (typeof counterpartyRef !== 'function') requireRef(counterpartyRef, 'counterpartyRef')
  const records = []

  function middleware (req, res, next) {
    const signature = readHeader(req.headers, H_SIGNATURE)
    if (signature === null) return next()

    const signatureSeenAt = isoNow(now)
    const chunks = []
    const keep = (chunk, encoding) => {
      if (chunk === undefined || chunk === null || typeof chunk === 'function') return
      chunks.push(typeof chunk === 'string' ? encodeText(chunk, encoding) : new Uint8Array(chunk))
    }
    const write = res.write
    const end = res.end
    res.write = function (chunk, encoding, cb) {
      keep(chunk, encoding)
      return write.call(this, chunk, encoding, cb)
    }
    res.end = function (chunk, encoding, cb) {
      keep(chunk, encoding)
      const result = end.call(this, chunk, encoding, cb)
      const record = {
        schema: EXCHANGE_SCHEMA,
        side: 'seller',
        recorder_ref: recorderRef,
        counterparty_ref: typeof counterpartyRef === 'function' ? requireRef(counterpartyRef(req), 'counterpartyRef(req)') : counterpartyRef,
        request: { method: String(req.method).toUpperCase(), url: req.originalUrl ?? req.url },
        payment_required_header: null,
        payment_signature_header: signature,
        response_status: res.statusCode,
        payment_response_header: readHeader(res.getHeaders(), H_RESPONSE),
        response_body_b64: bytesToBase64(concat(chunks)),
        signature_seen_at: signatureSeenAt,
        response_seen_at: isoNow(now)
      }
      records.push(record)
      if (onRecord) onRecord(record)
      return result
    }
    next()
  }

  return { middleware, records }
}

// UTF-8 by hand: Bare has no TextEncoder (measured), and the recorder must load there.
function encodeText (text, encoding) {
  if (typeof encoding === 'string' && !/^utf-?8$/i.test(encoding)) {
    throw new Error(`acta x402 recorder: unsupported response encoding ${encoding}`)
  }
  const out = []
  for (const char of text) {
    const code = char.codePointAt(0)
    if (code < 0x80) out.push(code)
    else if (code < 0x800) out.push(0xc0 | (code >> 6), 0x80 | (code & 63))
    else if (code < 0x10000) out.push(0xe0 | (code >> 12), 0x80 | ((code >> 6) & 63), 0x80 | (code & 63))
    else out.push(0xf0 | (code >> 18), 0x80 | ((code >> 12) & 63), 0x80 | ((code >> 6) & 63), 0x80 | (code & 63))
  }
  return new Uint8Array(out)
}

function concat (chunks) {
  let length = 0
  for (const chunk of chunks) length += chunk.length
  const out = new Uint8Array(length)
  let offset = 0
  for (const chunk of chunks) {
    out.set(chunk, offset)
    offset += chunk.length
  }
  return out
}

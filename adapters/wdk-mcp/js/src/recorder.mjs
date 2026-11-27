// ACTA MCP session recorder (implementation candidate, Proposed ADR-021).
//
// Sits between an AI client and an MCP server that speaks newline-delimited JSON-RPC over
// stdio — here Tether's `wdk-mcp` wallet server — and keeps every `tools/call` exactly as it
// crossed: the request line and the response line, byte for byte.
//
// It records. It does not approve, block, limit or rewrite anything. A line is parsed only to
// learn which call it belongs to; what is kept is the line itself.
//
// No dependencies and no Node built-ins, so the same file loads on Node.js and on Bare.

export const SESSION_SCHEMA = 'acta.mcp.session.v0'

function requireText (value, name) {
  if (typeof value !== 'string' || value.trim() === '') throw new TypeError(`${name} must be a non-empty string`)
  return value
}

/**
 * `declared` is the deployer's own statement about the server's tools. The recorder does not
 * check it against the server; the mapper reads it and says so.
 *
 *   significantTools   tools whose execution moves value or produces a signature
 *   previewArguments   { tool: { argument, default } } for tools that preview unless told not to
 *   denialCodes        error codes that mean "a control refused", as opposed to "it failed"
 */
export function createSessionRecorder ({
  sessionId,
  agentRef,
  deployedByRef,
  standingInstruction,
  declared = {},
  now = Date.now
} = {}) {
  const record = {
    schema: SESSION_SCHEMA,
    session_id: requireText(sessionId, 'sessionId'),
    agent_ref: requireText(agentRef, 'agentRef'),
    deployed_by_ref: requireText(deployedByRef, 'deployedByRef'),
    standing_instruction: requireText(standingInstruction, 'standingInstruction'),
    declared: {
      significant_tools: [...(declared.significantTools ?? [])],
      preview_arguments: { ...(declared.previewArguments ?? {}) },
      denial_codes: [...(declared.denialCodes ?? [])]
    },
    opened_at: new Date(now()).toISOString(),
    closed_at: null,
    initialize: null,
    tool_catalog: null,
    calls: [],
    unparsed_lines: 0
  }
  const pending = new Map()
  const stamp = () => new Date(now()).toISOString()
  const key = (id) => JSON.stringify(id)

  function parse (line) {
    try {
      const message = JSON.parse(line)
      return message !== null && typeof message === 'object' ? message : null
    } catch {
      return null
    }
  }

  /** A line the client sent to the server. */
  function clientLine (line) {
    const message = parse(line)
    if (message === null) { record.unparsed_lines++; return }
    if (message.id === undefined || typeof message.method !== 'string') return
    if (message.method === 'initialize' || message.method === 'tools/list') {
      const entry = { request: line, response: null, requested_at: stamp(), responded_at: null }
      pending.set(key(message.id), entry)
      if (message.method === 'initialize') record.initialize = entry
      else record.tool_catalog = entry
    } else if (message.method === 'tools/call') {
      const entry = {
        jsonrpc_id: key(message.id),
        tool: typeof message.params?.name === 'string' ? message.params.name : '',
        request: line,
        response: null,
        requested_at: stamp(),
        responded_at: null
      }
      pending.set(key(message.id), entry)
      record.calls.push(entry)
    }
  }

  /** A line the server sent to the client. */
  function serverLine (line) {
    const message = parse(line)
    if (message === null) { record.unparsed_lines++; return }
    if (message.id === undefined || typeof message.method === 'string') return
    const entry = pending.get(key(message.id))
    if (!entry) return
    entry.response = line
    entry.responded_at = stamp()
    pending.delete(key(message.id))
  }

  function close () {
    if (record.closed_at === null) record.closed_at = stamp()
    return record
  }

  return { clientLine, serverLine, close, record }
}

/**
 * Splits a stream into lines without touching what is forwarded. Bytes are split on the
 * newline byte and only whole lines are decoded, so a multi-byte character cut by a chunk
 * boundary is never decoded in halves.
 */
export function createLineSplitter (onLine) {
  let rest = new Uint8Array(0)
  const emit = (bytes) => {
    const line = decodeUtf8(bytes).replace(/\r$/, '')
    if (line.trim() !== '') onLine(line)
  }
  return {
    push (chunk) {
      const bytes = typeof chunk === 'string' ? encodeUtf8(chunk) : new Uint8Array(chunk.buffer ?? chunk, chunk.byteOffset ?? 0, chunk.byteLength ?? chunk.length)
      const joined = new Uint8Array(rest.length + bytes.length)
      joined.set(rest)
      joined.set(bytes, rest.length)
      let start = 0
      let end
      while ((end = joined.indexOf(10, start)) >= 0) {
        emit(joined.subarray(start, end))
        start = end + 1
      }
      rest = joined.slice(start)
    },
    flush () {
      if (rest.length > 0) emit(rest)
      rest = new Uint8Array(0)
    }
  }
}

// UTF-8 by hand: Bare has neither TextEncoder nor TextDecoder (measured).
function encodeUtf8 (text) {
  const out = []
  for (const char of text) {
    const code = char.codePointAt(0)
    if (code < 0x80) out.push(code)
    else if (code < 0x800) out.push(0xc0 | (code >> 6), 0x80 | (code & 63))
    else if (code < 0x10000) out.push(0xe0 | (code >> 12), 0x80 | ((code >> 6) & 63), 0x80 | (code & 63))
    else out.push(0xf0 | (code >> 18), 0x80 | ((code >> 12) & 63), 0x80 | ((code >> 6) & 63), 0x80 | (code & 63))
  }
  return Uint8Array.from(out)
}

function decodeUtf8 (bytes) {
  let out = ''
  for (let i = 0; i < bytes.length;) {
    const a = bytes[i++]
    let code
    if (a < 0x80) code = a
    else if (a < 0xe0) code = ((a & 31) << 6) | (bytes[i++] & 63)
    else if (a < 0xf0) code = ((a & 15) << 12) | ((bytes[i++] & 63) << 6) | (bytes[i++] & 63)
    else code = ((a & 7) << 18) | ((bytes[i++] & 63) << 12) | ((bytes[i++] & 63) << 6) | (bytes[i++] & 63)
    out += String.fromCodePoint(code)
  }
  return out
}

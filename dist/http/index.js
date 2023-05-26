// compiled/http/snippets/wasm-streams-42e57edbcd526312/inline0.js
function bytes_literal() {
  return "bytes";
}

// compiled/http/index.js
var heap = new Array(128).fill(void 0);
heap.push(void 0, null, true, false);
var heap_next = heap.length;
function addHeapObject(obj) {
  if (heap_next === heap.length)
    heap.push(heap.length + 1);
  const idx = heap_next;
  heap_next = heap[idx];
  heap[idx] = obj;
  return idx;
}
function getObject(idx) {
  return heap[idx];
}
function isLikeNone(x) {
  return x === void 0 || x === null;
}
var cachedFloat64Memory0 = null;
function getFloat64Memory0() {
  if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
    cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
  }
  return cachedFloat64Memory0;
}
var cachedInt32Memory0 = null;
function getInt32Memory0() {
  if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
    cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
  }
  return cachedInt32Memory0;
}
var WASM_VECTOR_LEN = 0;
var cachedUint8Memory0 = null;
function getUint8Memory0() {
  if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
    cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8Memory0;
}
var cachedTextEncoder = typeof TextEncoder !== "undefined" ? new TextEncoder("utf-8") : { encode: () => {
  throw Error("TextEncoder not available");
} };
var encodeString = function(arg, view) {
  return cachedTextEncoder.encodeInto(arg, view);
};
function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === void 0) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr2 = malloc(buf.length) >>> 0;
    getUint8Memory0().subarray(ptr2, ptr2 + buf.length).set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr2;
  }
  let len = arg.length;
  let ptr = malloc(len) >>> 0;
  const mem = getUint8Memory0();
  let offset = 0;
  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 127)
      break;
    mem[ptr + offset] = code;
  }
  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }
    ptr = realloc(ptr, len, len = offset + arg.length * 3) >>> 0;
    const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
    const ret = encodeString(arg, view);
    offset += ret.written;
  }
  WASM_VECTOR_LEN = offset;
  return ptr;
}
var cachedTextDecoder = typeof TextDecoder !== "undefined" ? new TextDecoder("utf-8", { ignoreBOM: true, fatal: true }) : { decode: () => {
  throw Error("TextDecoder not available");
} };
if (typeof TextDecoder !== "undefined") {
  cachedTextDecoder.decode();
}
function getStringFromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
function dropObject(idx) {
  if (idx < 132)
    return;
  heap[idx] = heap_next;
  heap_next = idx;
}
function takeObject(idx) {
  const ret = getObject(idx);
  dropObject(idx);
  return ret;
}
var cachedBigInt64Memory0 = null;
function getBigInt64Memory0() {
  if (cachedBigInt64Memory0 === null || cachedBigInt64Memory0.byteLength === 0) {
    cachedBigInt64Memory0 = new BigInt64Array(wasm.memory.buffer);
  }
  return cachedBigInt64Memory0;
}
function debugString(val) {
  const type = typeof val;
  if (type == "number" || type == "boolean" || val == null) {
    return `${val}`;
  }
  if (type == "string") {
    return `"${val}"`;
  }
  if (type == "symbol") {
    const description = val.description;
    if (description == null) {
      return "Symbol";
    } else {
      return `Symbol(${description})`;
    }
  }
  if (type == "function") {
    const name = val.name;
    if (typeof name == "string" && name.length > 0) {
      return `Function(${name})`;
    } else {
      return "Function";
    }
  }
  if (Array.isArray(val)) {
    const length = val.length;
    let debug = "[";
    if (length > 0) {
      debug += debugString(val[0]);
    }
    for (let i = 1; i < length; i++) {
      debug += ", " + debugString(val[i]);
    }
    debug += "]";
    return debug;
  }
  const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
  let className;
  if (builtInMatches.length > 1) {
    className = builtInMatches[1];
  } else {
    return toString.call(val);
  }
  if (className == "Object") {
    try {
      return "Object(" + JSON.stringify(val) + ")";
    } catch (_) {
      return "Object";
    }
  }
  if (val instanceof Error) {
    return `${val.name}: ${val.message}
${val.stack}`;
  }
  return className;
}
function makeMutClosure(arg0, arg1, dtor, f) {
  const state = { a: arg0, b: arg1, cnt: 1, dtor };
  const real = (...args) => {
    state.cnt++;
    const a = state.a;
    state.a = 0;
    try {
      return f(a, state.b, ...args);
    } finally {
      if (--state.cnt === 0) {
        wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);
      } else {
        state.a = a;
      }
    }
  };
  real.original = state;
  return real;
}
function __wbg_adapter_48(arg0, arg1, arg2) {
  wasm.__wbindgen_export_3(arg0, arg1, addHeapObject(arg2));
}
function setup() {
  wasm.setup();
}
function handleError(f, args) {
  try {
    return f.apply(this, args);
  } catch (e) {
    wasm.__wbindgen_export_4(addHeapObject(e));
  }
}
function __wbg_adapter_134(arg0, arg1, arg2, arg3) {
  wasm.__wbindgen_export_5(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}
function notDefined(what) {
  return () => {
    throw new Error(`${what} is not defined`);
  };
}
var IntoUnderlyingByteSource = class {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_intounderlyingbytesource_free(ptr);
  }
  /**
  * @returns {any}
  */
  get type() {
    const ret = wasm.intounderlyingbytesource_type(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
  * @returns {number}
  */
  get autoAllocateChunkSize() {
    const ret = wasm.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
  * @param {any} controller
  */
  start(controller) {
    wasm.intounderlyingbytesource_start(this.__wbg_ptr, addHeapObject(controller));
  }
  /**
  * @param {any} controller
  * @returns {Promise<any>}
  */
  pull(controller) {
    const ret = wasm.intounderlyingbytesource_pull(this.__wbg_ptr, addHeapObject(controller));
    return takeObject(ret);
  }
  /**
  */
  cancel() {
    const ptr = this.__destroy_into_raw();
    wasm.intounderlyingbytesource_cancel(ptr);
  }
};
var IntoUnderlyingSink = class {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_intounderlyingsink_free(ptr);
  }
  /**
  * @param {any} chunk
  * @returns {Promise<any>}
  */
  write(chunk) {
    const ret = wasm.intounderlyingsink_write(this.__wbg_ptr, addHeapObject(chunk));
    return takeObject(ret);
  }
  /**
  * @returns {Promise<any>}
  */
  close() {
    const ptr = this.__destroy_into_raw();
    const ret = wasm.intounderlyingsink_close(ptr);
    return takeObject(ret);
  }
  /**
  * @param {any} reason
  * @returns {Promise<any>}
  */
  abort(reason) {
    const ptr = this.__destroy_into_raw();
    const ret = wasm.intounderlyingsink_abort(ptr, addHeapObject(reason));
    return takeObject(ret);
  }
};
var IntoUnderlyingSource = class {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_intounderlyingsource_free(ptr);
  }
  /**
  * @param {any} controller
  * @returns {Promise<any>}
  */
  pull(controller) {
    const ret = wasm.intounderlyingsource_pull(this.__wbg_ptr, addHeapObject(controller));
    return takeObject(ret);
  }
  /**
  */
  cancel() {
    const ptr = this.__destroy_into_raw();
    wasm.intounderlyingsource_cancel(ptr);
  }
};
var PipeOptions = class {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_pipeoptions_free(ptr);
  }
  /**
  * @returns {boolean}
  */
  get preventClose() {
    const ret = wasm.pipeoptions_preventClose(this.__wbg_ptr);
    return ret !== 0;
  }
  /**
  * @returns {boolean}
  */
  get preventCancel() {
    const ret = wasm.pipeoptions_preventCancel(this.__wbg_ptr);
    return ret !== 0;
  }
  /**
  * @returns {boolean}
  */
  get preventAbort() {
    const ret = wasm.pipeoptions_preventAbort(this.__wbg_ptr);
    return ret !== 0;
  }
  /**
  * @returns {AbortSignal | undefined}
  */
  get signal() {
    const ret = wasm.pipeoptions_signal(this.__wbg_ptr);
    return takeObject(ret);
  }
};
var QueuingStrategy = class {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_queuingstrategy_free(ptr);
  }
  /**
  * @returns {number}
  */
  get highWaterMark() {
    const ret = wasm.queuingstrategy_highWaterMark(this.__wbg_ptr);
    return ret;
  }
};
var ReadableStreamGetReaderOptions = class {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_readablestreamgetreaderoptions_free(ptr);
  }
  /**
  * @returns {any}
  */
  get mode() {
    const ret = wasm.readablestreamgetreaderoptions_mode(this.__wbg_ptr);
    return takeObject(ret);
  }
};
var Surreal = class {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(Surreal.prototype);
    obj.__wbg_ptr = ptr;
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_surreal_free(ptr);
  }
  /**
  * Construct the database engine
  *
  * ```js
  * const db = new Surreal();
  * ```
  */
  constructor() {
    const ret = wasm.surreal_init();
    return Surreal.__wrap(ret);
  }
  /**
  * Connect to a database engine
  *
  * ```js
  * const db = new Surreal();
  *
  * // Connect to a WebSocket engine
  * await db.connect('ws://localhost:8000');
  *
  * // Connect to an HTTP engine
  * await db.connect('http://localhost:8000');
  *
  * // Connect to a memory engine
  * await db.connect('mem://');
  *
  * // Connect to an IndxDB engine
  * await db.connect('indxdb://MyDatabase');
  *
  * // Connect to a strict memory engine
  * await db.connect('memory', { strict: true });
  *
  * // Limit number of concurrent connections
  * await db.connect('ws://localhost:8000', { capacity: 100000 });
  * ```
  * @param {string} endpoint
  * @param {any} opts
  * @returns {Promise<void>}
  */
  connect(endpoint, opts) {
    const ptr0 = passStringToWasm0(endpoint, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.surreal_connect(this.__wbg_ptr, ptr0, len0, addHeapObject(opts));
    return takeObject(ret);
  }
  /**
  * Switch to a specific namespace or database
  *
  * ```js
  * const db = new Surreal();
  *
  * // Switch to a namespace
  * await db.use({ ns: 'namespace' });
  *
  * // Switch to a database
  * await db.use({ db: 'database' });
  *
  * // Switch both
  * await db.use({ ns: 'namespace', db: 'database' });
  * ```
  * @param {any} value
  * @returns {Promise<void>}
  */
  use(value) {
    const ret = wasm.surreal_use(this.__wbg_ptr, addHeapObject(value));
    return takeObject(ret);
  }
  /**
  * Assign a value as a parameter for this connection
  *
  * ```js
  * await db.set('name', { first: 'Tobie', last: 'Morgan Hitchcock' });
  * ```
  * @param {string} key
  * @param {any} value
  * @returns {Promise<void>}
  */
  set(key, value) {
    const ptr0 = passStringToWasm0(key, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.surreal_set(this.__wbg_ptr, ptr0, len0, addHeapObject(value));
    return takeObject(ret);
  }
  /**
  * Remove a parameter from this connection
  *
  * ```js
  * await db.unset('name');
  * ```
  * @param {string} key
  * @returns {Promise<void>}
  */
  unset(key) {
    const ptr0 = passStringToWasm0(key, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.surreal_unset(this.__wbg_ptr, ptr0, len0);
    return takeObject(ret);
  }
  /**
  * Sign up a user to a specific authentication scope
  *
  * ```js
  * const token = await db.signup({
  *     namespace: 'namespace',
  *     database: 'database',
  *     scope: 'user_scope',
  *     email: 'john.doe@example.com',
  *     password: 'password123'
  * });
  * ```
  * @param {any} credentials
  * @returns {Promise<any>}
  */
  signup(credentials) {
    const ret = wasm.surreal_signup(this.__wbg_ptr, addHeapObject(credentials));
    return takeObject(ret);
  }
  /**
  * Sign this connection in to a specific authentication scope
  *
  * ```js
  * const token = await db.signin({
  *     namespace: 'namespace',
  *     database: 'database',
  *     scope: 'user_scope',
  *     email: 'john.doe@example.com',
  *     password: 'password123'
  * });
  * ```
  * @param {any} credentials
  * @returns {Promise<any>}
  */
  signin(credentials) {
    const ret = wasm.surreal_signin(this.__wbg_ptr, addHeapObject(credentials));
    return takeObject(ret);
  }
  /**
  * Invalidates the authentication for the current connection
  *
  * ```js
  * await db.invalidate();
  * ```
  * @returns {Promise<void>}
  */
  invalidate() {
    const ret = wasm.surreal_invalidate(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
  * Authenticates the current connection with a JWT token
  *
  * ```js
  * await db.authenticate('<secret token>');
  * ```
  * @param {string} token
  * @returns {Promise<void>}
  */
  authenticate(token) {
    const ptr0 = passStringToWasm0(token, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.surreal_authenticate(this.__wbg_ptr, ptr0, len0);
    return takeObject(ret);
  }
  /**
  * Run a SurrealQL query against the database
  *
  * ```js
  * // Run a query without bindings
  * const people = await db.query('SELECT * FROM person');
  *
  * // Run a query with bindings
  * const people = await db.query('SELECT * FROM type::table($table)', { table: 'person' });
  * ```
  * @param {string} sql
  * @param {any} bindings
  * @returns {Promise<any>}
  */
  query(sql, bindings) {
    const ptr0 = passStringToWasm0(sql, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.surreal_query(this.__wbg_ptr, ptr0, len0, addHeapObject(bindings));
    return takeObject(ret);
  }
  /**
  * Select all records in a table, or a specific record
  *
  * ```js
  * // Select all records from a table
  * const people = await db.select('person');
  *
  * // Select a range records from a table
  * const people = await db.select('person:jane..john');
  *
  * // Select a specific record from a table
  * const person = await db.select('person:h5wxrf2ewk8xjxosxtyc');
  * ```
  * @param {string} resource
  * @returns {Promise<any>}
  */
  select(resource) {
    const ptr0 = passStringToWasm0(resource, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.surreal_select(this.__wbg_ptr, ptr0, len0);
    return takeObject(ret);
  }
  /**
  * Create a record in the database
  *
  * ```js
  * // Create a record with no fields set
  * const person = await db.create('person');
  *
  * Create a record with fields set
  * const person = await db.create('person', {
  *     name: 'Tobie',
  *     settings: {
  *         active: true,
  *         marketing: true
  *     }
  * });
  * ```
  * @param {string} resource
  * @param {any} data
  * @returns {Promise<any>}
  */
  create(resource, data) {
    const ptr0 = passStringToWasm0(resource, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.surreal_create(this.__wbg_ptr, ptr0, len0, addHeapObject(data));
    return takeObject(ret);
  }
  /**
  * Update all records in a table, or a specific record
  *
  * ```js
  * // Replace all records in a table with the specified data.
  * const people = await db.update('person', {
  *     name: 'Tobie',
  *     settings: {
  *         active: true,
  *         marketing: true
  *     }
  * });
  *
  * // Replace a range of records with the specified data.
  * const person = await db.update('person:jane..john', {
  *     name: 'Tobie',
  *     settings: {
  *         active: true,
  *         marketing: true
  *     }
  * });
  *
  * // Replace the current document / record data with the specified data.
  * const person = await db.update('person:tobie', {
  *     name: 'Tobie',
  *     settings: {
  *         active: true,
  *         marketing: true
  *     }
  * });
  * ```
  * @param {string} resource
  * @param {any} data
  * @returns {Promise<any>}
  */
  update(resource, data) {
    const ptr0 = passStringToWasm0(resource, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.surreal_update(this.__wbg_ptr, ptr0, len0, addHeapObject(data));
    return takeObject(ret);
  }
  /**
  * Merge records in a table with specified data
  *
  * ```js
  * // Merge all records in a table with specified data.
  * const person = await db.merge('person', {
  *     marketing: true
  * });
  *
  * // Merge a range of records with the specified data.
  * const person = await db.merge('person:jane..john', {
  *     marketing: true
  * });
  *
  * // Merge the current document / record data with the specified data.
  * const person = await db.merge('person:tobie', {
  *     marketing: true
  * });
  * ```
  * @param {string} resource
  * @param {any} data
  * @returns {Promise<any>}
  */
  merge(resource, data) {
    const ptr0 = passStringToWasm0(resource, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.surreal_merge(this.__wbg_ptr, ptr0, len0, addHeapObject(data));
    return takeObject(ret);
  }
  /**
  * Patch all records in a table or a specific record
  *
  * ```js
  * // Apply JSON Patch changes to all records in the database.
  * const person = await db.patch('person', [{
  *     op: 'replace',
  *     path: '/settings/active',
  *     value: false
  * }]);
  *
  * // Apply JSON Patch to a range of records.
  * const person = await db.patch('person:jane..john', [{
  *     op: 'replace',
  *     path: '/settings/active',
  *     value: false
  * }]);
  *
  * // Apply JSON Patch to a specific record.
  * const person = await db.patch('person:tobie', [{
  *     op: 'replace',
  *     path: '/settings/active',
  *     value: false
  * }]);
  * ```
  * @param {string} resource
  * @param {any} data
  * @returns {Promise<any>}
  */
  patch(resource, data) {
    const ptr0 = passStringToWasm0(resource, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.surreal_patch(this.__wbg_ptr, ptr0, len0, addHeapObject(data));
    return takeObject(ret);
  }
  /**
  * Delete all records, or a specific record
  *
  * ```js
  * // Delete all records from a table
  * const records = await db.delete('person');
  *
  * // Delete a range records from a table
  * const people = await db.delete('person:jane..john');
  *
  * // Delete a specific record from a table
  * const record = await db.delete('person:h5wxrf2ewk8xjxosxtyc');
  * ```
  * @param {string} resource
  * @returns {Promise<any>}
  */
  delete(resource) {
    const ptr0 = passStringToWasm0(resource, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.surreal_delete(this.__wbg_ptr, ptr0, len0);
    return takeObject(ret);
  }
  /**
  * Return the version of the server
  *
  * ```js
  * const version = await db.version();
  * ```
  * @returns {Promise<any>}
  */
  version() {
    const ret = wasm.surreal_version(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
  * Check whether the server is healthy or not
  *
  * ```js
  * await db.health();
  * ```
  * @returns {Promise<void>}
  */
  health() {
    const ret = wasm.surreal_health(this.__wbg_ptr);
    return takeObject(ret);
  }
};
var imports = {
  __wbindgen_placeholder__: {
    __wbg_iterator_7c7e58f62eb84700: function() {
      const ret = Symbol.iterator;
      return addHeapObject(ret);
    },
    __wbg_length_820c786973abdd8a: function(arg0) {
      const ret = getObject(arg0).length;
      return ret;
    },
    __wbindgen_number_new: function(arg0) {
      const ret = arg0;
      return addHeapObject(ret);
    },
    __wbg_new_0394642eae39db16: function() {
      const ret = new Array();
      return addHeapObject(ret);
    },
    __wbg_set_b4da98d504ac6091: function(arg0, arg1, arg2) {
      getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
    },
    __wbg_new_0f2b71ca2f2a6029: function() {
      const ret = /* @__PURE__ */ new Map();
      return addHeapObject(ret);
    },
    __wbg_new_2b6fea4ea03b1b95: function() {
      const ret = new Object();
      return addHeapObject(ret);
    },
    __wbindgen_is_string: function(arg0) {
      const ret = typeof getObject(arg0) === "string";
      return ret;
    },
    __wbg_set_841ac57cff3d672b: function(arg0, arg1, arg2) {
      getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
    },
    __wbg_set_da7be7bf0e037b14: function(arg0, arg1, arg2) {
      const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
      return addHeapObject(ret);
    },
    __wbindgen_bigint_from_i64: function(arg0) {
      const ret = arg0;
      return addHeapObject(ret);
    },
    __wbindgen_bigint_from_u64: function(arg0) {
      const ret = BigInt.asUintN(64, arg0);
      return addHeapObject(ret);
    },
    __wbindgen_is_undefined: function(arg0) {
      const ret = getObject(arg0) === void 0;
      return ret;
    },
    __wbindgen_in: function(arg0, arg1) {
      const ret = getObject(arg0) in getObject(arg1);
      return ret;
    },
    __wbindgen_number_get: function(arg0, arg1) {
      const obj = getObject(arg1);
      const ret = typeof obj === "number" ? obj : void 0;
      getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
      getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    },
    __wbindgen_boolean_get: function(arg0) {
      const v = getObject(arg0);
      const ret = typeof v === "boolean" ? v ? 1 : 0 : 2;
      return ret;
    },
    __wbindgen_string_get: function(arg0, arg1) {
      const obj = getObject(arg1);
      const ret = typeof obj === "string" ? obj : void 0;
      var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
      var len1 = WASM_VECTOR_LEN;
      getInt32Memory0()[arg0 / 4 + 1] = len1;
      getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    },
    __wbindgen_is_bigint: function(arg0) {
      const ret = typeof getObject(arg0) === "bigint";
      return ret;
    },
    __wbindgen_is_object: function(arg0) {
      const val = getObject(arg0);
      const ret = typeof val === "object" && val !== null;
      return ret;
    },
    __wbindgen_error_new: function(arg0, arg1) {
      const ret = new Error(getStringFromWasm0(arg0, arg1));
      return addHeapObject(ret);
    },
    __wbindgen_string_new: function(arg0, arg1) {
      const ret = getStringFromWasm0(arg0, arg1);
      return addHeapObject(ret);
    },
    __wbg_getwithrefkey_5e6d9547403deab8: function(arg0, arg1) {
      const ret = getObject(arg0)[getObject(arg1)];
      return addHeapObject(ret);
    },
    __wbindgen_jsval_eq: function(arg0, arg1) {
      const ret = getObject(arg0) === getObject(arg1);
      return ret;
    },
    __wbg_String_88810dfeb4021902: function(arg0, arg1) {
      const ret = String(getObject(arg1));
      const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
      const len1 = WASM_VECTOR_LEN;
      getInt32Memory0()[arg0 / 4 + 1] = len1;
      getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    },
    __wbg_new_2b55e405e4af4986: function(arg0, arg1) {
      try {
        var state0 = { a: arg0, b: arg1 };
        var cb0 = (arg02, arg12) => {
          const a = state0.a;
          state0.a = 0;
          try {
            return __wbg_adapter_134(a, state0.b, arg02, arg12);
          } finally {
            state0.a = a;
          }
        };
        const ret = new Promise(cb0);
        return addHeapObject(ret);
      } finally {
        state0.a = state0.b = 0;
      }
    },
    __wbindgen_memory: function() {
      const ret = wasm.memory;
      return addHeapObject(ret);
    },
    __wbg_buffer_55ba7a6b1b92e2ac: function(arg0) {
      const ret = getObject(arg0).buffer;
      return addHeapObject(ret);
    },
    __wbg_newwithbyteoffsetandlength_88d1d8be5df94b9b: function(arg0, arg1, arg2) {
      const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
      return addHeapObject(ret);
    },
    __wbg_randomFillSync_e950366c42764a07: function() {
      return handleError(function(arg0, arg1) {
        getObject(arg0).randomFillSync(takeObject(arg1));
      }, arguments);
    },
    __wbg_subarray_d82be056deb4ad27: function(arg0, arg1, arg2) {
      const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
      return addHeapObject(ret);
    },
    __wbg_getRandomValues_3774744e221a22ad: function() {
      return handleError(function(arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
      }, arguments);
    },
    __wbg_crypto_70a96de3b6b73dac: function(arg0) {
      const ret = getObject(arg0).crypto;
      return addHeapObject(ret);
    },
    __wbg_process_dd1577445152112e: function(arg0) {
      const ret = getObject(arg0).process;
      return addHeapObject(ret);
    },
    __wbg_versions_58036bec3add9e6f: function(arg0) {
      const ret = getObject(arg0).versions;
      return addHeapObject(ret);
    },
    __wbg_node_6a9d28205ed5b0d8: function(arg0) {
      const ret = getObject(arg0).node;
      return addHeapObject(ret);
    },
    __wbg_require_f05d779769764e82: function() {
      return handleError(function() {
        const ret = module.require;
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_msCrypto_adbc770ec9eca9c7: function(arg0) {
      const ret = getObject(arg0).msCrypto;
      return addHeapObject(ret);
    },
    __wbg_newwithlength_89eeca401d8918c2: function(arg0) {
      const ret = new Uint8Array(arg0 >>> 0);
      return addHeapObject(ret);
    },
    __wbindgen_is_function: function(arg0) {
      const ret = typeof getObject(arg0) === "function";
      return ret;
    },
    __wbg_call_557a2f2deacc4912: function() {
      return handleError(function(arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_get_7303ed2ef026b2f5: function(arg0, arg1) {
      const ret = getObject(arg0)[arg1 >>> 0];
      return addHeapObject(ret);
    },
    __wbg_next_ec061e48a0e72a96: function() {
      return handleError(function(arg0) {
        const ret = getObject(arg0).next();
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_done_b6abb27d42b63867: function(arg0) {
      const ret = getObject(arg0).done;
      return ret;
    },
    __wbg_value_2f4ef2036bfad28e: function(arg0) {
      const ret = getObject(arg0).value;
      return addHeapObject(ret);
    },
    __wbg_get_f53c921291c381bd: function() {
      return handleError(function(arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_next_f4bc0e96ea67da68: function(arg0) {
      const ret = getObject(arg0).next;
      return addHeapObject(ret);
    },
    __wbindgen_object_clone_ref: function(arg0) {
      const ret = getObject(arg0);
      return addHeapObject(ret);
    },
    __wbg_self_742dd6eab3e9211e: function() {
      return handleError(function() {
        const ret = self.self;
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_window_c409e731db53a0e2: function() {
      return handleError(function() {
        const ret = window.window;
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_globalThis_b70c095388441f2d: function() {
      return handleError(function() {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_global_1c72617491ed7194: function() {
      return handleError(function() {
        const ret = global.global;
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_newnoargs_c9e6043b8ad84109: function(arg0, arg1) {
      const ret = new Function(getStringFromWasm0(arg0, arg1));
      return addHeapObject(ret);
    },
    __wbg_isArray_04e59fb73f78ab5b: function(arg0) {
      const ret = Array.isArray(getObject(arg0));
      return ret;
    },
    __wbg_call_587b30eea3e09332: function() {
      return handleError(function(arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_isSafeInteger_2088b01008075470: function(arg0) {
      const ret = Number.isSafeInteger(getObject(arg0));
      return ret;
    },
    __wbg_set_07da13cc24b69217: function() {
      return handleError(function(arg0, arg1, arg2) {
        const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
      }, arguments);
    },
    __wbg_new_09938a7d020f049b: function(arg0) {
      const ret = new Uint8Array(getObject(arg0));
      return addHeapObject(ret);
    },
    __wbg_set_3698e3ca519b3c3c: function(arg0, arg1, arg2) {
      getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    },
    __wbg_length_0aab7ffd65ad19ed: function(arg0) {
      const ret = getObject(arg0).length;
      return ret;
    },
    __wbindgen_jsval_loose_eq: function(arg0, arg1) {
      const ret = getObject(arg0) == getObject(arg1);
      return ret;
    },
    __wbg_instanceof_Uint8Array_1349640af2da2e88: function(arg0) {
      let result;
      try {
        result = getObject(arg0) instanceof Uint8Array;
      } catch {
        result = false;
      }
      const ret = result;
      return ret;
    },
    __wbg_instanceof_ArrayBuffer_ef2632aa0d4bfff8: function(arg0) {
      let result;
      try {
        result = getObject(arg0) instanceof ArrayBuffer;
      } catch {
        result = false;
      }
      const ret = result;
      return ret;
    },
    __wbg_entries_13e011453776468f: function(arg0) {
      const ret = Object.entries(getObject(arg0));
      return addHeapObject(ret);
    },
    __wbindgen_object_drop_ref: function(arg0) {
      takeObject(arg0);
    },
    __wbg_text_65fa1887e8f7b4ac: function() {
      return handleError(function(arg0) {
        const ret = getObject(arg0).text();
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_new_143b41b4342650bb: function() {
      return handleError(function() {
        const ret = new Headers();
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_new_668956ac1089f8cf: function() {
      return handleError(function() {
        const ret = new AbortController();
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_signal_bcb55e86063f8860: function(arg0) {
      const ret = getObject(arg0).signal;
      return addHeapObject(ret);
    },
    __wbg_newwithstrandinit_a4cd16dfaafcf625: function() {
      return handleError(function(arg0, arg1, arg2) {
        const ret = new Request(getStringFromWasm0(arg0, arg1), getObject(arg2));
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_has_40b8c976775c8ead: function() {
      return handleError(function(arg0, arg1) {
        const ret = Reflect.has(getObject(arg0), getObject(arg1));
        return ret;
      }, arguments);
    },
    __wbg_fetch_621998933558ad27: function(arg0, arg1) {
      const ret = getObject(arg0).fetch(getObject(arg1));
      return addHeapObject(ret);
    },
    __wbg_fetch_57429b87be3dcc33: function(arg0) {
      const ret = fetch(getObject(arg0));
      return addHeapObject(ret);
    },
    __wbg_append_fac652007989b765: function() {
      return handleError(function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).append(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
      }, arguments);
    },
    __wbg_instanceof_Response_7ade9a5a066d1a55: function(arg0) {
      let result;
      try {
        result = getObject(arg0) instanceof Response;
      } catch {
        result = false;
      }
      const ret = result;
      return ret;
    },
    __wbg_status_d2b2d0889f7e970f: function(arg0) {
      const ret = getObject(arg0).status;
      return ret;
    },
    __wbg_url_59cb32ef6a837521: function(arg0, arg1) {
      const ret = getObject(arg1).url;
      const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
      const len1 = WASM_VECTOR_LEN;
      getInt32Memory0()[arg0 / 4 + 1] = len1;
      getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    },
    __wbg_headers_2de03c88f895093b: function(arg0) {
      const ret = getObject(arg0).headers;
      return addHeapObject(ret);
    },
    __wbg_stringify_d06ad2addc54d51e: function() {
      return handleError(function(arg0) {
        const ret = JSON.stringify(getObject(arg0));
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_arrayBuffer_2693673868da65b7: function() {
      return handleError(function(arg0) {
        const ret = getObject(arg0).arrayBuffer();
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_now_5eac63e226f04e45: typeof Date.now == "function" ? Date.now : notDefined("Date.now"),
    __wbindgen_bigint_get_as_i64: function(arg0, arg1) {
      const v = getObject(arg1);
      const ret = typeof v === "bigint" ? v : void 0;
      getBigInt64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? BigInt(0) : ret;
      getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    },
    __wbindgen_debug_string: function(arg0, arg1) {
      const ret = debugString(getObject(arg1));
      const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
      const len1 = WASM_VECTOR_LEN;
      getInt32Memory0()[arg0 / 4 + 1] = len1;
      getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    },
    __wbindgen_throw: function(arg0, arg1) {
      throw new Error(getStringFromWasm0(arg0, arg1));
    },
    __wbindgen_cb_drop: function(arg0) {
      const obj = takeObject(arg0).original;
      if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
      }
      const ret = false;
      return ret;
    },
    __wbg_then_8df675b8bb5d5e3c: function(arg0, arg1) {
      const ret = getObject(arg0).then(getObject(arg1));
      return addHeapObject(ret);
    },
    __wbg_then_835b073a479138e5: function(arg0, arg1, arg2) {
      const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
      return addHeapObject(ret);
    },
    __wbg_resolve_ae38ad63c43ff98b: function(arg0) {
      const ret = Promise.resolve(getObject(arg0));
      return addHeapObject(ret);
    },
    __wbg_respond_f4778bef04e912a6: function(arg0, arg1) {
      getObject(arg0).respond(arg1 >>> 0);
    },
    __wbg_byobRequest_a3c74c3694777d1b: function(arg0) {
      const ret = getObject(arg0).byobRequest;
      return isLikeNone(ret) ? 0 : addHeapObject(ret);
    },
    __wbg_close_a41954830b65c455: function(arg0) {
      getObject(arg0).close();
    },
    __wbg_enqueue_3a8a8e67e44d2567: function(arg0, arg1) {
      getObject(arg0).enqueue(getObject(arg1));
    },
    __wbg_view_d1a31268af734e5d: function(arg0) {
      const ret = getObject(arg0).view;
      return isLikeNone(ret) ? 0 : addHeapObject(ret);
    },
    __wbg_byteLength_1fef7842ca4200fa: function(arg0) {
      const ret = getObject(arg0).byteLength;
      return ret;
    },
    __wbg_close_045ed342139beb7d: function(arg0) {
      getObject(arg0).close();
    },
    __wbg_new_87297f22973157c8: function(arg0, arg1) {
      const ret = new Error(getStringFromWasm0(arg0, arg1));
      return addHeapObject(ret);
    },
    __wbg_buffer_610b70c8fd30da2d: function(arg0) {
      const ret = getObject(arg0).buffer;
      return addHeapObject(ret);
    },
    __wbg_byteOffset_ede786cfcf88d3dd: function(arg0) {
      const ret = getObject(arg0).byteOffset;
      return ret;
    },
    __wbg_bytesliteral_efe7d360639bf32b: function() {
      const ret = bytes_literal();
      return addHeapObject(ret);
    },
    __wbg_abort_de75e4ab5136bcee: function(arg0) {
      getObject(arg0).abort();
    },
    __wbg_debug_e3f6a1578e6d45ca: function(arg0) {
      console.debug(getObject(arg0));
    },
    __wbg_error_a7e23606158b68b9: function(arg0) {
      console.error(getObject(arg0));
    },
    __wbg_info_05db236d79f1b785: function(arg0) {
      console.info(getObject(arg0));
    },
    __wbg_log_dc06ec929fc95a20: function(arg0) {
      console.log(getObject(arg0));
    },
    __wbg_warn_9bdd743e9f5fe1e0: function(arg0) {
      console.warn(getObject(arg0));
    },
    __wbindgen_closure_wrapper4596: function(arg0, arg1, arg2) {
      const ret = makeMutClosure(arg0, arg1, 265, __wbg_adapter_48);
      return addHeapObject(ret);
    }
  }
};
var wasm_url = new URL("index_bg.wasm", import.meta.url);
var wasmCode = "";
switch (wasm_url.protocol) {
  case "file:":
    wasmCode = await Deno.readFile(wasm_url);
    break;
  case "https:":
  case "http:":
    wasmCode = await (await fetch(wasm_url)).arrayBuffer();
    break;
  default:
    throw new Error(`Unsupported protocol: ${wasm_url.protocol}`);
}
var wasmInstance = (await WebAssembly.instantiate(wasmCode, imports)).instance;
var wasm = wasmInstance.exports;
export {
  IntoUnderlyingByteSource,
  IntoUnderlyingSink,
  IntoUnderlyingSource,
  PipeOptions,
  QueuingStrategy,
  ReadableStreamGetReaderOptions,
  Surreal,
  setup
};
//# sourceMappingURL=index.js.map

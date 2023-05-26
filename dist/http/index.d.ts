/* tslint:disable */
/* eslint-disable */
/**
*/
export function setup(): void;
/**
*/
export class IntoUnderlyingByteSource {
  free(): void;
/**
* @param {any} controller
*/
  start(controller: any): void;
/**
* @param {any} controller
* @returns {Promise<any>}
*/
  pull(controller: any): Promise<any>;
/**
*/
  cancel(): void;
/**
*/
  readonly autoAllocateChunkSize: number;
/**
*/
  readonly type: any;
}
/**
*/
export class IntoUnderlyingSink {
  free(): void;
/**
* @param {any} chunk
* @returns {Promise<any>}
*/
  write(chunk: any): Promise<any>;
/**
* @returns {Promise<any>}
*/
  close(): Promise<any>;
/**
* @param {any} reason
* @returns {Promise<any>}
*/
  abort(reason: any): Promise<any>;
}
/**
*/
export class IntoUnderlyingSource {
  free(): void;
/**
* @param {any} controller
* @returns {Promise<any>}
*/
  pull(controller: any): Promise<any>;
/**
*/
  cancel(): void;
}
/**
* Raw options for [`pipeTo()`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeTo).
*/
export class PipeOptions {
  free(): void;
/**
*/
  readonly preventAbort: boolean;
/**
*/
  readonly preventCancel: boolean;
/**
*/
  readonly preventClose: boolean;
/**
*/
  readonly signal: AbortSignal | undefined;
}
/**
*/
export class QueuingStrategy {
  free(): void;
/**
*/
  readonly highWaterMark: number;
}
/**
* Raw options for [`getReader()`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/getReader).
*/
export class ReadableStreamGetReaderOptions {
  free(): void;
/**
*/
  readonly mode: any;
}
/**
*/
export class Surreal {
  free(): void;
/**
* Construct the database engine
*
* ```js
* const db = new Surreal();
* ```
*/
  constructor();
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
  connect(endpoint: string, opts: any): Promise<void>;
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
  use(value: any): Promise<void>;
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
  set(key: string, value: any): Promise<void>;
/**
* Remove a parameter from this connection
*
* ```js
* await db.unset('name');
* ```
* @param {string} key
* @returns {Promise<void>}
*/
  unset(key: string): Promise<void>;
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
  signup(credentials: any): Promise<any>;
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
  signin(credentials: any): Promise<any>;
/**
* Invalidates the authentication for the current connection
*
* ```js
* await db.invalidate();
* ```
* @returns {Promise<void>}
*/
  invalidate(): Promise<void>;
/**
* Authenticates the current connection with a JWT token
*
* ```js
* await db.authenticate('<secret token>');
* ```
* @param {string} token
* @returns {Promise<void>}
*/
  authenticate(token: string): Promise<void>;
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
  query(sql: string, bindings: any): Promise<any>;
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
  select(resource: string): Promise<any>;
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
  create(resource: string, data: any): Promise<any>;
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
  update(resource: string, data: any): Promise<any>;
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
  merge(resource: string, data: any): Promise<any>;
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
  patch(resource: string, data: any): Promise<any>;
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
  delete(resource: string): Promise<any>;
/**
* Return the version of the server
*
* ```js
* const version = await db.version();
* ```
* @returns {Promise<any>}
*/
  version(): Promise<any>;
/**
* Check whether the server is healthy or not
*
* ```js
* await db.health();
* ```
* @returns {Promise<void>}
*/
  health(): Promise<void>;
}

// @deno-types="../dist/embedded/index.d.ts"
import { ConnectionOptions, SurrealWasmEngine as Swe } from "../dist/embedded";
import { ConnectionStatus, ConnectionUnavailable, decodeCbor, Emitter, encodeCbor, Engine, Engines, EngineEvents, RpcRequest, RpcResponse, UnexpectedConnectionError, UnexpectedServerResponse } from "surrealdb.js";
import { getIncrementalID } from "./getIncrementalId";
import z from 'zod';

export function surrealdbWasmEngines(opts?: ConnectionOptions) {
	class WasmEmbeddedEngine implements Engine {
		ready?: Promise<void>;
		reader?: Promise<void>;
		status: ConnectionStatus = ConnectionStatus.Disconnected;
		connection: {
			url?: URL;
			namespace?: string;
			database?: string;
			token?: string;
		} = {};

		readonly emitter: Emitter<EngineEvents>;
		private db?: Swe;

		constructor(emitter: Emitter<EngineEvents>) {
			this.emitter = emitter;
		}

		private setStatus<T extends ConnectionStatus>(
			status: T,
			...args: EngineEvents[T]
		) {
			this.status = status;
			this.emitter.emit(status, args);
		}

		async connect(url: URL) {
			this.connection.url = url;
			this.setStatus(ConnectionStatus.Connecting);
			const db = await Swe.connect(url.toString(), opts).catch(e => {
				console.log(e);
				const error = new UnexpectedConnectionError(
					typeof e == 'string' ? e : "error" in e ? e.error : "An unexpected error occurred",
				);
				this.setStatus(ConnectionStatus.Error, error);
				throw e;
			});

			this.reader = (async () => {
				const reader = db.notifications().getReader();
				while (this.connected) {
					const { done, value } = await reader.read();
					if (done) break;
					const raw = value as Uint8Array;
					const { id, action, result } = decodeCbor(raw.buffer);
					if (id) this.emitter.emit(`live-${id.toString()}`, [action, result], true);
				}
			})();

			this.db = db;
			this.setStatus(ConnectionStatus.Connected)
		}

		async disconnect(): Promise<void> {
			this.connection = {};
			await this.ready;
			if (!this.db) throw new ConnectionUnavailable();
			delete this.db;
			await this.reader;
			delete this.reader;
		}

		async rpc<
			Method extends string,
			Params extends unknown[] | undefined,
			Result extends unknown,
		>(request: RpcRequest<Method, Params>): Promise<RpcResponse<Result>> {
			await this.ready;
			if (!this.db) throw new ConnectionUnavailable();

			// It's not realistic for the message to ever arrive before the listener is registered on the emitter
			// And we don't want to collect the response messages in the emitter
			// So to be sure we simply subscribe before we send the message :)

			const id = getIncrementalID();
			const res: RpcResponse = await this.db.execute(new Uint8Array(encodeCbor({ id, ...request })))
				.then(raw => ({ result: decodeCbor(raw.buffer) }))
				.catch(message => ({ error: { code: -1, message } }));

			if ("result" in res) {
				switch (request.method) {
					case "use": {
						this.connection.namespace = z.string().parse(
							request.params?.[0],
						);
						this.connection.database = z.string().parse(
							request.params?.[1],
						);
						break;
					}

					case "signin":
					case "signup": {
						this.connection.token = res.result as string;
						break;
					}

					case "authenticate": {
						this.connection.token = request.params
							?.[0] as string;
						break;
					}

					case "invalidate": {
						delete this.connection.token;
						break;
					}
				}
			}

			return res as RpcResponse<Result>;
		}

		get connected() {
			return !!this.db;
		}
	}

	return {
		mem: WasmEmbeddedEngine,
		indxdb: WasmEmbeddedEngine,
	}
}

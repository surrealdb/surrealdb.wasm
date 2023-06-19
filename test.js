import { Surreal } from './dist/full/index.js';

const db = new Surreal()
await db.connect('memory');
await db.use({ ns: 'test', db: 'test' });
await db.create('test')

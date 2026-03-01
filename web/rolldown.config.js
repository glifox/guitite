import { defineConfig } from 'rolldown';
import { dts } from 'rolldown-plugin-dts'

export default defineConfig([
  {
    input: 'guitite.ts',
    external: ['loro-crdt'],
    plugins: [dts()],
    output: {
      dir: 'dist',
      format: 'esm',
    }
  }
]);
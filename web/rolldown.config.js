import { defineConfig } from 'rolldown';
import { dts } from 'rolldown-plugin-dts'

const isProduction = process.env.NODE_ENV === 'production';

export default defineConfig([
  {
    input: 'guitite.ts',
    external: ['loro-crdt'],
    plugins: [dts()],
    platform: "browser",
    output: {
      dir: 'dist',
      format: 'esm',
      minify: isProduction,
    }
  }
]);
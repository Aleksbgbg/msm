import vue from "@vitejs/plugin-vue";
import { defineConfig } from "vite";
import svgLoader from "vite-svg-loader";
import tsconfigPaths from "vite-tsconfig-paths";

// https://vitejs.dev/config/
export default defineConfig({
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // Tauri expects a fixed port, fail if that port is not available
  server: {
    strictPort: true,
  },
  // Make use of
  // `TAURI_PLATFORM`, `TAURI_ARCH`, `TAURI_FAMILY`,
  // `TAURI_PLATFORM_VERSION`, `TAURI_PLATFORM_TYPE` and `TAURI_DEBUG`
  // env variables
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    // Tauri supports es2021
    target: ["es2021", "chrome100", "safari13"],
    // Don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // Produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  // resolve: {
  //   alias: [
  //     { find: '@', replacement: path.resolve(__dirname, 'src') },
  //   ],
  // },
  resolve: {
    alias: [{ find: "@", replacement: "/src" }],
  },
  plugins: [vue(), svgLoader(), tsconfigPaths()],
});

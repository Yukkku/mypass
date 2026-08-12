import { cpSync, rmSync } from "fs";

rmSync("./dist", { force: true, recursive: true });

await Bun.build({
  entrypoints: ['./src/background.js'],
  outdir: './dist',
  minify: true,
});

cpSync("./manifest.json", "./dist/manifest.json");

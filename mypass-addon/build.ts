import { cpSync, rmSync } from "fs";

rmSync("./dist", { force: true, recursive: true });

await Bun.build({
  entrypoints: [
    "./src/background.ts",
    "./src/content.ts",
    "./src/popup.html",
  ],
  outdir: "./dist",
  format: "cjs",
  // minify: true,
});

cpSync("./manifest.json", "./dist/manifest.json");

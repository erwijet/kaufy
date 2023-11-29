const fs = require("fs");
const { mkdir, writeFile } = require("fs/promises");
const { Readable } = require("stream");
const { finished } = require("stream/promises");
const { resolve } = require("path");

async function download() {
  const dest = resolve("./schema.graphql");
  const res = await fetch("https://api.kaufy.holewinski.dev/schema");

  if (fs.existsSync(dest)) fs.unlinkSync(dest);

  const fstream = fs.createWriteStream(dest, { flags: "wx" });
  await finished(Readable.fromWeb(res.body).pipe(fstream));
}

download().then(() => process.exit(0));

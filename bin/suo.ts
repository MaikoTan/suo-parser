#!/usr/bin/env node

import { transformFile } from "../";
const file = process.argv[2];

transformFile(file, (err, result) => {
  if (err) {
    console.error(err);
    process.exit(1);
  }

  console.log(result);
});

import test from "node:test";
import * as assert from "node:assert";

import { getLoadablePath } from "./src/index.js";
import { basename, extname, isAbsolute } from "node:path";

import Database from "better-sqlite3";
import sqlite3 from "sqlite3";

test("getLoadblePath()", (t) => {
  const loadablePath = getLoadablePath();
  assert.strictEqual(isAbsolute(loadablePath), true);
  assert.strictEqual(basename(loadablePath, extname(loadablePath)), "regex0");
});

test("better-sqlite3", (t) => {
  const db = new Database(":memory:");
  db.loadExtension(getLoadablePath());
  const version = db.prepare("select regex_version()").pluck().get();
  assert.strictEqual(version[0], "v");
});

test("sqlite3", async (t) => {
  const db = new sqlite3.Database(":memory:");
  db.loadExtension(getLoadablePath());
  let version = await new Promise((resolve, reject) => {
    db.get("select regex_version()", (err, row) => {
      if (err) return reject(err);
      resolve(row["regex_version()"]);
    });
  });
  assert.strictEqual(version[0], "v");
});

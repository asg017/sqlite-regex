import { download } from "https://deno.land/x/plug@1.0.1/mod.ts";
import meta from "./deno.json" assert { type: "json" };

const BASE = `${meta.github}/releases/download/v${meta.version}`;

// Similar to https://github.com/denodrivers/sqlite3/blob/f7529897720631c2341b713f0d78d4d668593ea9/src/ffi.ts#L561
let path: string;
try {
  const customPath = Deno.env.get("DENO_SQLITE_REGEX_PATH");
  if (customPath) path = customPath;
  else {
    path = await download({
      url: {
        darwin: {
          aarch64: `${BASE}/deno-darwin-aarch64.regex0.dylib`,
          x86_64: `${BASE}/deno-darwin-x86_64.regex0.dylib`,
        },
        windows: {
          x86_64: `${BASE}/deno-windows-x86_64.regex0.dll`,
        },
        linux: {
          x86_64: `${BASE}/deno-linux-x86_64.regex0.so`,
        },
      },
      suffixes: {
        darwin: "",
        linux: "",
        windows: "",
      },
    });
  }
} catch (e) {
  if (e instanceof Deno.errors.PermissionDenied) {
    throw e;
  }

  const error = new Error("Failed to load sqlite-regex extension");
  error.cause = e;

  throw error;
}

/**
 * Returns the full path to the compiled sqlite-regex extension.
 * Caution: this will not be named "regex0.dylib|so|dll", since plug will
 * replace the name with a hash.
 */
export function getLoadablePath(): string {
  return path;
}

/**
 * Entrypoint name for the sqlite-regex extension.
 */
export const entrypoint = "sqlite3_regex_init";

interface Db {
  // after https://deno.land/x/sqlite3@0.8.0/mod.ts?s=Database#method_loadExtension_0
  loadExtension(file: string, entrypoint?: string | undefined): void;
}
/**
 * Loads the sqlite-regex extension on the given sqlite3 database.
 */
export function load(db: Db): void {
  db.loadExtension(path, entrypoint);
}

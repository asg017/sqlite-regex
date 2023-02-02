# The `datasette-sqlite-regex` Datasette Plugin

`datasette-sqlite-regex` is a [Datasette plugin](https://docs.datasette.io/en/stable/plugins.html) that loads the [`sqlite-regex`](https://github.com/asg017/sqlite-regex) extension in Datasette instances, allowing you to generate and work with [regexs](https://github.com/regex/spec) in SQL.

```
datasette install datasette-sqlite-regex
```

See [`docs.md`](../../docs.md) for a full API reference for the TODO SQL functions.

Alternatively, when publishing Datasette instances, you can use the `--install` option to install the plugin.

```
datasette publish cloudrun data.db --service=my-service --install=datasette-sqlite-regex

```

# swc-plugin-no-console

remove `console` function when use swc.

## Usage

```json
{
  "$schema": "https://json.schemastore.org/swcrc",
  "jsc": {
    "parser": {
      "syntax": "ecmascript"
    },
    "experimental": {
      "plugins": [["swc-plugin-no-console", {
        "excludes": ["log"],
        "includesValue": ["data"]
      }]]
    }
  },
}
```

`excludes`: methods in `excludes` are preserved

`includesValue`: output containing any value from `includes_value` is preserved.(only string now)

## TODO
- [ ] support `excludes_files`
- [ ] support any type in `includes_value`
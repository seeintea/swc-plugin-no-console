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
        "includesValue": ["data"],
        "excludesFiles": ["index.js"]
      }]]
    }
  },
}
```

`excludes`: methods in `excludes` are preserved

`includesValue`: output containing any value from `includes_value` is preserved.(only basic type)

`excludesFiles`: files in `excludes_files` are not processed
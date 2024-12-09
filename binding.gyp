{
  "targets": [
    {
      "target_name": "tree_sitter_redscript_binding",
      "dependencies": [
        "<!(node -p \"require('node-addon-api').targets\"):node_addon_api_except",
      ],
      "include_dirs": [
        "src",
      ],
      "sources": [
        "src/parser.c",
        # NOTE: if your language has an external scanner, add it here.
        "src/scanner.c",
      ],
      "cflags_c": [
        "-std=c11",
      ]
    }
  ]
}


(function_declaration
  body: (_) @function.inside) @function.around

(function_declaration
	name: (simple_identifier) @_name
	(#match? @_name "^test")
)

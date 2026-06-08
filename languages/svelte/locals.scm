[
  (document)
  (element)
  (if_block)
  (else_if_clause)
  (else_clause)
  (each_block)
  (await_block)
  (await_pending)
  (await_branch)
  (await_branch_children)
  (key_block)
  (orphan_branch)
  (snippet_block)
] @local.scope

(each_block
  binding: (pattern
    content: (_) @local.definition))

(each_block
  index: (pattern
    content: (_) @local.definition))

(await_block
  binding: (pattern
    content: (_) @local.definition))

(await_branch
  (pattern
    content: (_) @local.definition))

(orphan_branch
  (pattern
    content: (_) @local.definition))

(snippet_block
  name: (snippet_name) @local.definition.function)

(snippet_parameters
  parameter: (pattern
    content: (_) @local.definition))

((const_tag
  expression: (expression_value
    content: (_) @local.definition))
  (#match? @local.definition "\\S"))

((declaration_tag
  declaration: (expression_value
    content: (_) @local.definition))
  (#match? @local.definition "\\S"))

((attribute
  name: (attribute_name
    (attribute_directive) @_directive
    (attribute_identifier) @local.definition))
  (#eq? @_directive "let"))

((attribute
  name: (attribute_name
    (attribute_directive) @_directive
    (attribute_identifier) @local.reference))
  (#any-of? @_directive "use" "transition" "in" "out" "animate"))

((attribute
  name: (attribute_name
    (attribute_directive) @_directive
    (attribute_identifier) @local.reference)
  !value)
  (#any-of? @_directive "bind" "class" "style")
  (#match? @local.reference "^[A-Za-z_$][A-Za-z0-9_$]*$"))

(expression
  content: (_) @local.reference)

(else_if_clause
  expression: (expression_value
    content: (_) @local.reference))

(orphan_branch
  expression: (expression_value
    content: (_) @local.reference))

(attach_tag
  expression: (expression_value
    content: (_) @local.reference))

(debug_tag
  expression: (expression_value
    content: (_) @local.reference))

(html_tag
  expression: (expression_value
    content: (_) @local.reference))

(render_tag
  expression: (expression_value
    content: (_) @local.reference))

(shorthand_attribute
  content: (_) @local.reference)

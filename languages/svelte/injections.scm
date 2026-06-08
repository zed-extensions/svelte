; Script elements with TypeScript
((element
  (start_tag
    (tag_name) @_tag
    (attribute
      (attribute_name) @_script_attr
      (quoted_attribute_value
        (attribute_value) @_value)))
  (raw_text) @injection.content)
  (#match? @_tag "^[Ss][Cc][Rr][Ii][Pp][Tt]$")
  (#any-of? @_script_attr "lang" "type")
  (#any-of? @_value "ts" "typescript" "text/typescript" "application/typescript")
  (#set! injection.language "typescript"))

; Script elements with explicit JavaScript
((element
  (start_tag
    (tag_name) @_tag
    (attribute
      (attribute_name) @_script_attr
      (quoted_attribute_value
        (attribute_value) @_value)))
  (raw_text) @injection.content)
  (#match? @_tag "^[Ss][Cc][Rr][Ii][Pp][Tt]$")
  (#any-of? @_script_attr "lang" "type")
  (#any-of? @_value "js" "javascript" "text/javascript" "application/javascript" "module")
  (#set! injection.language "javascript"))

; Script content (JavaScript default)
((element
  (start_tag
    (tag_name) @_tag
    (attribute
      (attribute_name) @_script_attr)*)
  (raw_text) @injection.content)
  (#match? @_tag "^[Ss][Cc][Rr][Ii][Pp][Tt]$")
  (#not-any-of? @_script_attr "lang" "type")
  (#set! injection.language "javascript"))

; Style with lang="scss"
((element
  (start_tag
    (tag_name) @_tag
    (attribute
      (attribute_name) @_lang
      (quoted_attribute_value
        (attribute_value) @_scss)))
  (raw_text) @injection.content)
  (#eq? @_tag "style")
  (#eq? @_lang "lang")
  (#eq? @_scss "scss")
  (#set! injection.language "scss"))

; Style with lang="sass"
((element
  (start_tag
    (tag_name) @_tag
    (attribute
      (attribute_name) @_lang
      (quoted_attribute_value
        (attribute_value) @_sass)))
  (raw_text) @injection.content)
  (#eq? @_tag "style")
  (#eq? @_lang "lang")
  (#eq? @_sass "sass")
  (#set! injection.language "sass"))

; Style with lang="less"
((element
  (start_tag
    (tag_name) @_tag
    (attribute
      (attribute_name) @_lang
      (quoted_attribute_value
        (attribute_value) @_less)))
  (raw_text) @injection.content)
  (#eq? @_tag "style")
  (#eq? @_lang "lang")
  (#eq? @_less "less")
  (#set! injection.language "less"))

; Style with explicit CSS
((element
  (start_tag
    (tag_name) @_tag
    (attribute
      (attribute_name) @_lang
      (quoted_attribute_value
        (attribute_value) @_css)))
  (raw_text) @injection.content)
  (#eq? @_tag "style")
  (#eq? @_lang "lang")
  (#eq? @_css "css")
  (#set! injection.language "css"))

; Style content (CSS default — only when no lang attribute)
((element
  (start_tag
    (tag_name) @_tag
    (attribute
      (attribute_name) @_style_attr)*)
  (raw_text) @injection.content)
  (#eq? @_tag "style")
  (#not-any-of? @_style_attr "lang")
  (#set! injection.language "css"))

; Inline style attribute
((attribute
  (attribute_name) @_style_name
  (quoted_attribute_value (attribute_value) @injection.content))
  (#eq? @_style_name "style")
  (#set! injection.language "css"))

; Typed expression content
((expression content: (js) @injection.content)
  (#set! injection.language "javascript"))
((expression content: (ts) @injection.content)
  (#set! injection.language "typescript"))

; Shorthand attributes ({foo})
((shorthand_attribute content: (js) @injection.content)
  (#set! injection.language "javascript"))
((shorthand_attribute content: (ts) @injection.content)
  (#set! injection.language "typescript"))

; Tag expressions ({@const}, {@render}, {@html}, {@debug}, {@attach}, {:else if})
((expression_value content: (js) @injection.content)
  (#set! injection.language "javascript"))
((expression_value content: (ts) @injection.content)
  (#set! injection.language "typescript"))

; Else-if clause
((else_if_clause expression: (expression_value content: (js) @injection.content))
  (#set! injection.language "javascript"))
((else_if_clause expression: (expression_value content: (ts) @injection.content))
  (#set! injection.language "typescript"))

; If block expressions
((if_block expression: (expression content: (js) @injection.content))
  (#set! injection.language "javascript"))
((if_block expression: (expression content: (ts) @injection.content))
  (#set! injection.language "typescript"))

; Each block expressions and bindings
((each_block expression: (expression content: (js) @injection.content))
  (#set! injection.language "javascript"))
((each_block expression: (expression content: (ts) @injection.content))
  (#set! injection.language "typescript"))
((each_block binding: (pattern content: (js) @injection.content))
  (#set! injection.language "javascript"))
((each_block binding: (pattern content: (ts) @injection.content))
  (#set! injection.language "typescript"))
((each_block index: (pattern content: (js) @injection.content))
  (#set! injection.language "javascript"))
((each_block index: (pattern content: (ts) @injection.content))
  (#set! injection.language "typescript"))
((each_block key: (expression content: (js) @injection.content))
  (#set! injection.language "javascript"))
((each_block key: (expression content: (ts) @injection.content))
  (#set! injection.language "typescript"))

; Await block expressions and bindings
((await_block expression: (expression content: (js) @injection.content))
  (#set! injection.language "javascript"))
((await_block expression: (expression content: (ts) @injection.content))
  (#set! injection.language "typescript"))
((await_block (pattern content: (js) @injection.content))
  (#set! injection.language "javascript"))
((await_block (pattern content: (ts) @injection.content))
  (#set! injection.language "typescript"))
((await_branch (pattern content: (js) @injection.content))
  (#set! injection.language "javascript"))
((await_branch (pattern content: (ts) @injection.content))
  (#set! injection.language "typescript"))
((orphan_branch (pattern content: (js) @injection.content))
  (#set! injection.language "javascript"))
((orphan_branch (pattern content: (ts) @injection.content))
  (#set! injection.language "typescript"))

; Key block expressions
((key_block expression: (expression content: (js) @injection.content))
  (#set! injection.language "javascript"))
((key_block expression: (expression content: (ts) @injection.content))
  (#set! injection.language "typescript"))

; Snippet parameters and type parameters
((snippet_parameters parameter: (pattern content: (js) @injection.content))
  (#set! injection.language "javascript"))
((snippet_parameters parameter: (pattern content: (ts) @injection.content))
  (#set! injection.language "typescript"))
((snippet_block type_parameters: (snippet_type_parameters) @injection.content)
  (#set! injection.language "typescript"))

; Comments
((comment) @injection.content
  (#set! injection.language "comment"))

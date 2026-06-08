(document) @item

(comment) @annotation

; Elements
(element
    (start_tag) @name
) @item

(element
    (self_closing_tag) @name
) @item

; Script elements
(element
    (start_tag
        (tag_name) @_tag
        (#match? @_tag "^[Ss][Cc][Rr][Ii][Pp][Tt]$"))
    (raw_text) @context
) @item

; Style elements
(element
    (start_tag
        (tag_name) @_tag
        (#match? @_tag "^[Ss][Tt][Yy][Ll][Ee]$"))
    (raw_text) @context
) @item

(if_block) @item

(else_clause) @item

(else_if_clause) @item

(each_block) @item

(await_block) @item

(await_branch) @item

(key_block) @item

(snippet_block
    name: (snippet_name) @name
) @item

(html_tag) @name @item

(const_tag) @name @item

(debug_tag) @name @item

(render_tag) @name @item

(attach_tag) @name @item

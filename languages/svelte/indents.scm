[
  (element)
  (if_block)
  (each_block)
  (await_block)
  (key_block)
  (snippet_block)
  (start_tag ">" @end)
  (self_closing_tag "/>" @end)
  (element
    (start_tag) @start
    [(end_tag) (erroneous_end_tag)]? @end)
] @indent

; ============================================================
; Markdown Codemap Query
; ============================================================

; ------------------------------------------------------------
; ATX Headings (# style)
; ------------------------------------------------------------

; H1
(atx_heading
  (atx_h1_marker)
  heading_content: (inline) @name) @definition.module

; H2
(atx_heading
  (atx_h2_marker)
  heading_content: (inline) @name) @definition.class

; H3-H6
(atx_heading
  [(atx_h3_marker) (atx_h4_marker) (atx_h5_marker) (atx_h6_marker)]
  heading_content: (inline) @name) @definition.function

; ------------------------------------------------------------
; Setext Headings (underline style)
; ------------------------------------------------------------

; H1 (=== underline)
(setext_heading
  heading_content: (_
    (inline) @name)
  (setext_h1_underline)) @definition.module

; H2 (--- underline)
(setext_heading
  heading_content: (_
    (inline) @name)
  (setext_h2_underline)) @definition.class

; ------------------------------------------------------------
; Fenced Code Blocks
; ------------------------------------------------------------
(fenced_code_block
  (info_string
    (language) @name)) @definition.namespace

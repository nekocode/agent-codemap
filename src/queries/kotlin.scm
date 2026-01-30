; ============================================================
; Kotlin Codemap Query (for tree-sitter-kotlin-ng)
; ============================================================

; ------------------------------------------------------------
; Interface (class_declaration with "interface" keyword)
; ------------------------------------------------------------
(class_declaration
  "interface" @_kw
  name: (identifier) @name) @definition.interface

; ------------------------------------------------------------
; Enum class (class_declaration with class_modifier containing "enum")
; Note: enum keyword is in class_modifier node, which is a child of modifiers
; ------------------------------------------------------------
(class_declaration
  (modifiers
    (class_modifier) @_enum)
  name: (identifier) @name
  (enum_class_body)) @definition.enum

; ------------------------------------------------------------
; Class (regular class, data class, sealed class etc.)
; ------------------------------------------------------------
(class_declaration
  "class" @_kw
  name: (identifier) @name) @definition.class

; ------------------------------------------------------------
; Object (singleton)
; ------------------------------------------------------------
(object_declaration
  name: (identifier) @name) @definition.class

; ------------------------------------------------------------
; Enum entries
; ------------------------------------------------------------
(enum_entry
  (identifier) @name) @definition.enum_member

; ------------------------------------------------------------
; Function
; ------------------------------------------------------------
(function_declaration
  name: (identifier) @name) @definition.function

; ------------------------------------------------------------
; Property
; ------------------------------------------------------------
(property_declaration
  (variable_declaration
    (identifier) @name)) @definition.property

; Class parameter (constructor property)
(class_parameter
  (identifier) @name) @definition.property

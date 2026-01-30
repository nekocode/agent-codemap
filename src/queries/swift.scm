; ============================================================
; Swift Codemap Query
; ============================================================

; ------------------------------------------------------------
; Class, Struct, Enum, Extension (unified in class_declaration)
; declaration_kind: class | struct | enum | extension | actor
; ------------------------------------------------------------

; Class
(class_declaration
  declaration_kind: "class"
  name: (_) @name) @definition.class

; Struct (mapped to class)
(class_declaration
  declaration_kind: "struct"
  name: (_) @name) @definition.class

; Enum
(class_declaration
  declaration_kind: "enum"
  name: (_) @name) @definition.enum

; Extension (namespace)
(class_declaration
  declaration_kind: "extension"
  name: (_) @name) @definition.namespace

; Actor (mapped to class)
(class_declaration
  declaration_kind: "actor"
  name: (_) @name) @definition.class

; ------------------------------------------------------------
; Protocol
; ------------------------------------------------------------
(protocol_declaration
  name: (type_identifier) @name) @definition.interface

; ------------------------------------------------------------
; Enum case
; ------------------------------------------------------------
(enum_entry
  name: (simple_identifier) @name) @definition.enum_member

; ------------------------------------------------------------
; Typealias
; ------------------------------------------------------------
(typealias_declaration
  name: (type_identifier) @name) @definition.type

; ------------------------------------------------------------
; Top-level Function
; ------------------------------------------------------------
(function_declaration
  name: (simple_identifier) @name) @definition.function

; ------------------------------------------------------------
; Initializer
; ------------------------------------------------------------
(init_declaration
  name: "init" @name) @definition.method

; ------------------------------------------------------------
; Property
; ------------------------------------------------------------
(property_declaration
  name: (pattern
    (simple_identifier) @name)) @definition.property

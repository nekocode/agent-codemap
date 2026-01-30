; ============================================================
; C# Codemap Query
; ============================================================

; ------------------------------------------------------------
; Namespace
; ------------------------------------------------------------
(namespace_declaration
  name: (identifier) @name) @definition.namespace

(namespace_declaration
  name: (qualified_name) @name) @definition.namespace

(file_scoped_namespace_declaration
  name: (identifier) @name) @definition.namespace

(file_scoped_namespace_declaration
  name: (qualified_name) @name) @definition.namespace

; ------------------------------------------------------------
; Interface
; ------------------------------------------------------------
(interface_declaration
  name: (identifier) @name) @definition.interface

; ------------------------------------------------------------
; Class
; ------------------------------------------------------------
(class_declaration
  name: (identifier) @name) @definition.class

; Record (C# 9+)
(record_declaration
  name: (identifier) @name) @definition.class

; ------------------------------------------------------------
; Struct
; ------------------------------------------------------------
(struct_declaration
  name: (identifier) @name) @definition.class

; ------------------------------------------------------------
; Enum
; ------------------------------------------------------------
(enum_declaration
  name: (identifier) @name) @definition.enum

; Enum members
(enum_member_declaration
  name: (identifier) @name) @definition.enum_member

; ------------------------------------------------------------
; Delegate
; ------------------------------------------------------------
(delegate_declaration
  name: (identifier) @name) @definition.type

; ------------------------------------------------------------
; Method
; ------------------------------------------------------------
(method_declaration
  name: (identifier) @name) @definition.method

; Constructor
(constructor_declaration
  name: (identifier) @name) @definition.method

; ------------------------------------------------------------
; Property
; ------------------------------------------------------------
(property_declaration
  name: (identifier) @name) @definition.property

; ------------------------------------------------------------
; Field
; ------------------------------------------------------------
(field_declaration
  (variable_declaration
    (variable_declarator
      (identifier) @name))) @definition.field

; ------------------------------------------------------------
; Event
; ------------------------------------------------------------
(event_declaration
  name: (identifier) @name) @definition.property

(event_field_declaration
  (variable_declaration
    (variable_declarator
      (identifier) @name))) @definition.property

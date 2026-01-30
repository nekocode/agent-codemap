; ============================================================
; Go Codemap Query
; ============================================================

; ------------------------------------------------------------
; 函数定义
; ------------------------------------------------------------
(function_declaration
  name: (identifier) @name) @definition.function

; ------------------------------------------------------------
; 方法定义
; ------------------------------------------------------------
(method_declaration
  name: (field_identifier) @name) @definition.method

; ------------------------------------------------------------
; Struct 定义 (必须在 type alias 之前)
; ------------------------------------------------------------
(type_declaration
  (type_spec
    name: (type_identifier) @name
    type: (struct_type) @_struct)) @definition.class

; Struct 字段
(field_declaration
  name: (field_identifier) @name) @definition.field

; ------------------------------------------------------------
; Interface 定义 (必须在 type alias 之前)
; ------------------------------------------------------------
(type_declaration
  (type_spec
    name: (type_identifier) @name
    type: (interface_type) @_iface)) @definition.interface

; Interface 方法签名
(method_elem
  name: (field_identifier) @name) @definition.method

; ------------------------------------------------------------
; Type Alias (简单类型别名)
; ------------------------------------------------------------
(type_declaration
  (type_spec
    name: (type_identifier) @name
    type: (type_identifier))) @definition.type

; 函数类型别名
(type_declaration
  (type_spec
    name: (type_identifier) @name
    type: (function_type))) @definition.type

; ------------------------------------------------------------
; 常量定义
; ------------------------------------------------------------
(const_spec
  name: (identifier) @name) @definition.constant

; ------------------------------------------------------------
; 变量定义
; ------------------------------------------------------------
(var_spec
  name: (identifier) @name) @definition.variable

; ============================================================
; Rust Codemap Query
; ============================================================

; ------------------------------------------------------------
; 函数定义
; ------------------------------------------------------------
(function_item
  name: (identifier) @name) @definition.function

; ------------------------------------------------------------
; 方法定义 (impl 块内)
; ------------------------------------------------------------
(impl_item
  body: (declaration_list
    (function_item
      name: (identifier) @name) @definition.method))

; ------------------------------------------------------------
; 结构体定义
; ------------------------------------------------------------
(struct_item
  name: (type_identifier) @name) @definition.class

; Struct 字段
(field_declaration
  name: (field_identifier) @name) @definition.field

; ------------------------------------------------------------
; 枚举定义
; ------------------------------------------------------------
(enum_item
  name: (type_identifier) @name) @definition.enum

; 枚举 variants
(enum_variant
  name: (identifier) @name) @definition.enum_member

; ------------------------------------------------------------
; Trait 定义
; ------------------------------------------------------------
(trait_item
  name: (type_identifier) @name) @definition.interface

; Trait 方法签名
(trait_item
  body: (declaration_list
    (function_signature_item
      name: (identifier) @name) @definition.method))

; Trait 默认方法实现
(trait_item
  body: (declaration_list
    (function_item
      name: (identifier) @name) @definition.method))

; ------------------------------------------------------------
; 类型别名
; ------------------------------------------------------------
(type_item
  name: (type_identifier) @name) @definition.type

; ------------------------------------------------------------
; 常量定义
; ------------------------------------------------------------
(const_item
  name: (identifier) @name) @definition.constant

; ------------------------------------------------------------
; 静态变量
; ------------------------------------------------------------
(static_item
  name: (identifier) @name) @definition.variable

; ------------------------------------------------------------
; 模块定义
; ------------------------------------------------------------
(mod_item
  name: (identifier) @name) @definition.module

; ------------------------------------------------------------
; 宏定义
; ------------------------------------------------------------
(macro_definition
  name: (identifier) @name) @definition.function

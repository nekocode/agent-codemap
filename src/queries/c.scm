; ============================================================
; C Codemap Query
; ============================================================

; ------------------------------------------------------------
; 函数定义
; ------------------------------------------------------------
(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name)) @definition.function

; 函数声明
(declaration
  declarator: (function_declarator
    declarator: (identifier) @name)) @definition.function

; 指针函数
(function_definition
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (identifier) @name))) @definition.function

; ------------------------------------------------------------
; 结构体定义
; ------------------------------------------------------------
(struct_specifier
  name: (type_identifier) @name
  body: (_) @_body) @definition.class

; Struct 字段
(field_declaration
  declarator: (field_identifier) @name) @definition.field

; Struct 指针字段
(field_declaration
  declarator: (pointer_declarator
    declarator: (field_identifier) @name)) @definition.field

; ------------------------------------------------------------
; 枚举定义
; ------------------------------------------------------------
(enum_specifier
  name: (type_identifier) @name
  body: (_) @_body) @definition.enum

; 枚举成员
(enumerator
  name: (identifier) @name) @definition.enum_member

; ------------------------------------------------------------
; 联合体定义
; ------------------------------------------------------------
(union_specifier
  name: (type_identifier) @name
  body: (_) @_body) @definition.class

; ------------------------------------------------------------
; 类型定义 (typedef)
; ------------------------------------------------------------
(type_definition
  declarator: (type_identifier) @name) @definition.type

; typedef struct
(type_definition
  type: (struct_specifier
    body: (_) @_body)
  declarator: (type_identifier) @name) @definition.class

; typedef enum
(type_definition
  type: (enum_specifier
    body: (_) @_body)
  declarator: (type_identifier) @name) @definition.enum

; ------------------------------------------------------------
; 宏定义
; ------------------------------------------------------------
(preproc_function_def
  name: (identifier) @name) @definition.function

; 宏常量
(preproc_def
  name: (identifier) @name) @definition.constant

; ------------------------------------------------------------
; 全局变量
; ------------------------------------------------------------
(declaration
  declarator: (identifier) @name) @definition.variable

(declaration
  declarator: (init_declarator
    declarator: (identifier) @name)) @definition.variable

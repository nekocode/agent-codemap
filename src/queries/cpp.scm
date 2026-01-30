; ============================================================
; C++ Codemap Query
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

; ------------------------------------------------------------
; 类定义
; ------------------------------------------------------------
(class_specifier
  name: (type_identifier) @name
  body: (_) @_body) @definition.class

; 结构体定义
(struct_specifier
  name: (type_identifier) @name
  body: (_) @_body) @definition.class

; ------------------------------------------------------------
; 类成员
; ------------------------------------------------------------

; 字段声明
(field_declaration
  declarator: (field_identifier) @name) @definition.field

; 指针字段
(field_declaration
  declarator: (pointer_declarator
    declarator: (field_identifier) @name)) @definition.field

; 方法定义 (类内)
(class_specifier
  body: (field_declaration_list
    (function_definition
      declarator: (function_declarator
        declarator: (field_identifier) @name)) @definition.method))

; 方法声明 (类内)
(field_declaration
  declarator: (function_declarator
    declarator: (field_identifier) @name)) @definition.method

; ------------------------------------------------------------
; 命名空间
; ------------------------------------------------------------
(namespace_definition
  name: (namespace_identifier) @name) @definition.namespace

; ------------------------------------------------------------
; 模板类
; ------------------------------------------------------------
(template_declaration
  (class_specifier
    name: (type_identifier) @name
    body: (_) @_body)) @definition.class

; 模板函数
(template_declaration
  (function_definition
    declarator: (function_declarator
      declarator: (identifier) @name))) @definition.function

; ------------------------------------------------------------
; 枚举
; ------------------------------------------------------------
(enum_specifier
  name: (type_identifier) @name
  body: (_) @_body) @definition.enum

; 枚举成员
(enumerator
  name: (identifier) @name) @definition.enum_member

; ------------------------------------------------------------
; 类型别名
; ------------------------------------------------------------
(alias_declaration
  name: (type_identifier) @name) @definition.type

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

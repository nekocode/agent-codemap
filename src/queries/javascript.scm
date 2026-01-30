; ============================================================
; JavaScript Codemap Query
; ============================================================

; ------------------------------------------------------------
; 类定义
; ------------------------------------------------------------
(class_declaration
  name: (identifier) @name) @definition.class

; 导出类
(export_statement
  (class_declaration
    name: (identifier) @name) @definition.class)

; ------------------------------------------------------------
; 类成员
; ------------------------------------------------------------

; 类字段 (ES2022 public fields)
(field_definition
  property: (property_identifier) @name) @definition.field

; 私有字段 (#field)
(field_definition
  property: (private_property_identifier) @name) @definition.field

; 静态字段
(field_definition
  property: (property_identifier) @name) @definition.field

; 方法定义
(method_definition
  name: (property_identifier) @name) @definition.method

; 私有方法
(method_definition
  name: (private_property_identifier) @name) @definition.method

; ------------------------------------------------------------
; 函数定义
; ------------------------------------------------------------
(function_declaration
  name: (identifier) @name) @definition.function

; 箭头函数 (const/let)
(lexical_declaration
  (variable_declarator
    name: (identifier) @name
    value: (arrow_function))) @definition.function

; 箭头函数 (var)
(variable_declaration
  (variable_declarator
    name: (identifier) @name
    value: (arrow_function))) @definition.function

; 导出函数
(export_statement
  (function_declaration
    name: (identifier) @name) @definition.function)

; 生成器函数
(generator_function_declaration
  name: (identifier) @name) @definition.function

; ------------------------------------------------------------
; 变量/常量
; ------------------------------------------------------------
(lexical_declaration
  (variable_declarator
    name: (identifier) @name) @definition.variable)

(variable_declaration
  (variable_declarator
    name: (identifier) @name) @definition.variable)

; ============================================================
; TypeScript Codemap Query
; ============================================================

; ------------------------------------------------------------
; 类定义
; ------------------------------------------------------------
(class_declaration
  name: (type_identifier) @name) @definition.class

(abstract_class_declaration
  name: (type_identifier) @name) @definition.class

; 导出类
(export_statement
  (class_declaration
    name: (type_identifier) @name) @definition.class)

; ------------------------------------------------------------
; 类成员
; ------------------------------------------------------------

; 类字段 (public/private/protected)
(public_field_definition
  name: (property_identifier) @name) @definition.field

; 私有字段 (#field)
(public_field_definition
  name: (private_property_identifier) @name) @definition.field

; 方法定义
(method_definition
  name: (property_identifier) @name) @definition.method

; 抽象方法
(abstract_method_signature
  name: (property_identifier) @name) @definition.method

; getter/setter
(method_definition
  name: (property_identifier) @name) @definition.method

; ------------------------------------------------------------
; 接口定义
; ------------------------------------------------------------
(interface_declaration
  name: (type_identifier) @name) @definition.interface

; 接口属性
(interface_declaration
  body: (interface_body
    (property_signature
      name: (property_identifier) @name) @definition.property))

; 接口方法签名
(interface_declaration
  body: (interface_body
    (method_signature
      name: (property_identifier) @name) @definition.method))

; ------------------------------------------------------------
; Type Alias
; ------------------------------------------------------------
(type_alias_declaration
  name: (type_identifier) @name) @definition.type

; ------------------------------------------------------------
; 枚举
; ------------------------------------------------------------
(enum_declaration
  name: (identifier) @name) @definition.enum

; 枚举成员 (带值)
(enum_assignment
  name: (property_identifier) @name) @definition.enum_member

; 枚举成员 (不带值)
(enum_body
  (property_identifier) @name @definition.enum_member)

; ------------------------------------------------------------
; 函数定义
; ------------------------------------------------------------
(function_declaration
  name: (identifier) @name) @definition.function

; 箭头函数 (变量赋值)
(lexical_declaration
  (variable_declarator
    name: (identifier) @name
    value: (arrow_function))) @definition.function

; 导出函数
(export_statement
  (function_declaration
    name: (identifier) @name) @definition.function)

; ------------------------------------------------------------
; 变量/常量
; ------------------------------------------------------------
(lexical_declaration
  (variable_declarator
    name: (identifier) @name
    value: (_) @_val) @definition.variable
  (#not-match? @_val "^\\("))

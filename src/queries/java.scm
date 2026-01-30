; ============================================================
; Java Codemap Query
; ============================================================

; ------------------------------------------------------------
; 类定义
; ------------------------------------------------------------
(class_declaration
  name: (identifier) @name) @definition.class

; ------------------------------------------------------------
; 接口定义
; ------------------------------------------------------------
(interface_declaration
  name: (identifier) @name) @definition.interface

; 接口常量
(interface_declaration
  body: (interface_body
    (constant_declaration
      declarator: (variable_declarator
        name: (identifier) @name)) @definition.constant))

; 接口方法签名
(interface_declaration
  body: (interface_body
    (method_declaration
      name: (identifier) @name) @definition.method))

; ------------------------------------------------------------
; 枚举定义
; ------------------------------------------------------------
(enum_declaration
  name: (identifier) @name) @definition.enum

; 枚举常量
(enum_constant
  name: (identifier) @name) @definition.enum_member

; 枚举字段
(enum_declaration
  body: (enum_body
    (enum_body_declarations
      (field_declaration
        declarator: (variable_declarator
          name: (identifier) @name)) @definition.field)))

; 枚举方法
(enum_declaration
  body: (enum_body
    (enum_body_declarations
      (method_declaration
        name: (identifier) @name) @definition.method)))

; ------------------------------------------------------------
; 方法定义
; ------------------------------------------------------------
(method_declaration
  name: (identifier) @name) @definition.method

; 构造函数
(constructor_declaration
  name: (identifier) @name) @definition.method

; ------------------------------------------------------------
; 字段定义
; ------------------------------------------------------------
(field_declaration
  declarator: (variable_declarator
    name: (identifier) @name)) @definition.field

; ------------------------------------------------------------
; 注解类型
; ------------------------------------------------------------
(annotation_type_declaration
  name: (identifier) @name) @definition.interface

; ------------------------------------------------------------
; 记录类 (Java 16+)
; ------------------------------------------------------------
(record_declaration
  name: (identifier) @name) @definition.class

; ============================================================
; Python Codemap Query
; ============================================================

; ------------------------------------------------------------
; 类定义
; ------------------------------------------------------------
(class_definition
  name: (identifier) @name) @definition.class

; ------------------------------------------------------------
; 类变量 (类体内的赋值)
; ------------------------------------------------------------
(class_definition
  body: (block
    (expression_statement
      (assignment
        left: (identifier) @name)) @definition.variable))

; ------------------------------------------------------------
; 方法定义
; ------------------------------------------------------------

; 普通方法
(class_definition
  body: (block
    (function_definition
      name: (identifier) @name) @definition.method))

; 装饰器方法 (@classmethod, @staticmethod, @property 等)
(class_definition
  body: (block
    (decorated_definition
      (function_definition
        name: (identifier) @name) @definition.method)))

; ------------------------------------------------------------
; 函数定义 (顶层)
; ------------------------------------------------------------
(module
  (function_definition
    name: (identifier) @name) @definition.function)

; 装饰器函数
(module
  (decorated_definition
    (function_definition
      name: (identifier) @name) @definition.function))

; ------------------------------------------------------------
; 模块级变量/常量
; ------------------------------------------------------------
(module
  (expression_statement
    (assignment
      left: (identifier) @name)) @definition.variable)

; ============================================================
; Objective-C Codemap Query
; ============================================================
;
; Note: ObjC grammar is inherited from C and has complex structures.
; We capture the main symbols but some edge cases may have duplicates.

; ------------------------------------------------------------
; Class Interface (@interface ClassName)
; Match the identifier that follows @interface keyword
; ------------------------------------------------------------
(class_interface
  "@interface" @_kw
  .
  (identifier) @name) @definition.class

; ------------------------------------------------------------
; Class Implementation (@implementation ClassName)
; ------------------------------------------------------------
(class_implementation
  "@implementation" @_kw
  .
  (identifier) @name) @definition.class

; ------------------------------------------------------------
; Protocol (@protocol ProtocolName)
; ------------------------------------------------------------
(protocol_declaration
  "@protocol" @_kw
  .
  (identifier) @name) @definition.interface

; ------------------------------------------------------------
; Method Definition (- (type)methodName or + (type)methodName)
; Match identifier in method_selector_no_list (simple method names)
; or keyword_declarator (for methods like initWithName:email:)
; ------------------------------------------------------------

; Simple method: - (void)doSomething
(method_definition
  (identifier) @name) @definition.method

; Method with keyword selector: - (id)initWithName:(NSString *)name
(method_definition
  (keyword_declarator
    (identifier) @name)) @definition.method

; Simple method declaration
(method_declaration
  (identifier) @name) @definition.method

; Method declaration with keyword selector
(method_declaration
  (keyword_declarator
    (identifier) @name)) @definition.method

; ------------------------------------------------------------
; C Functions
; ------------------------------------------------------------
(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name)) @definition.function

; ------------------------------------------------------------
; Property
; ------------------------------------------------------------
(property_declaration
  (struct_declaration
    (struct_declarator
      (identifier) @name))) @definition.property

(property_declaration
  (struct_declaration
    (struct_declarator
      (pointer_declarator
        declarator: (identifier) @name)))) @definition.property

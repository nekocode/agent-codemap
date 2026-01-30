// ============================================================
// Objective-C Test: Basic Symbols
// ============================================================

#import <Foundation/Foundation.h>

// ------------------------------------------------------------
// Protocol
// ------------------------------------------------------------
@protocol Identifiable <NSObject>

@required
- (NSString *)identifier;

@optional
- (void)identify;

@end

@protocol Cacheable

- (void)cache;
- (void)invalidate;

@end

// ------------------------------------------------------------
// Class Interface
// ------------------------------------------------------------
@interface User : NSObject <Identifiable>

@property (nonatomic, strong) NSString *userId;
@property (nonatomic, strong) NSString *name;
@property (nonatomic, strong) NSString *email;

- (instancetype)initWithName:(NSString *)name email:(NSString *)email;
- (BOOL)validate;
+ (instancetype)userWithName:(NSString *)name email:(NSString *)email;

@end

// ------------------------------------------------------------
// Class Implementation
// ------------------------------------------------------------
@implementation User

- (instancetype)initWithName:(NSString *)name email:(NSString *)email {
    self = [super init];
    if (self) {
        _name = name;
        _email = email;
        _userId = [[NSUUID UUID] UUIDString];
    }
    return self;
}

+ (instancetype)userWithName:(NSString *)name email:(NSString *)email {
    return [[self alloc] initWithName:name email:email];
}

- (BOOL)validate {
    return self.name.length > 0 && [self.email containsString:@"@"];
}

- (NSString *)identifier {
    return self.userId;
}

@end

// ------------------------------------------------------------
// Category
// ------------------------------------------------------------
@interface User (Display)

- (NSString *)displayName;

@end

@implementation User (Display)

- (NSString *)displayName {
    return [NSString stringWithFormat:@"%@ <%@>", self.name, self.email];
}

@end

// ------------------------------------------------------------
// Another Class
// ------------------------------------------------------------
@interface Address : NSObject

@property (nonatomic, strong) NSString *street;
@property (nonatomic, strong) NSString *city;
@property (nonatomic, strong) NSString *country;

@end

@implementation Address
@end

// ------------------------------------------------------------
// C Functions (also valid in ObjC)
// ------------------------------------------------------------
void greetUser(User *user) {
    NSLog(@"Hello, %@!", user.name);
}

BOOL validateEmail(NSString *email) {
    return [email containsString:@"@"];
}

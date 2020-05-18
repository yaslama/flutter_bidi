#import "BidiPlugin.h"
#if __has_include(<bidi/bidi-Swift.h>)
#import <bidi/bidi-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "bidi-Swift.h"
#endif

@implementation BidiPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftBidiPlugin registerWithRegistrar:registrar];
}
@end

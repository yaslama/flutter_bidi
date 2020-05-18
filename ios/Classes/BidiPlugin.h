#import <Flutter/Flutter.h>

@interface BidiPlugin : NSObject<FlutterPlugin>
@end
// NOTE: Append the lines below to ios/Classes/<your>Plugin.h

char *reorder(const char *to);

void rust_cstr_free(char *s);

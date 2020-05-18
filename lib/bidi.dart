import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

///////////////////////////////////////////////////////////////////////////////
// C bindings
///////////////////////////////////////////////////////////////////////////////

// void rust_cstr_free(char *s);
// char *reorder(const char *to);

///////////////////////////////////////////////////////////////////////////////
// Typedef's
///////////////////////////////////////////////////////////////////////////////

typedef ReorderFunc = Pointer<Utf8> Function(Pointer<Utf8>);
typedef ReorderFuncNative = Pointer<Utf8> Function(Pointer<Utf8>);

typedef FreeStringFunc = void Function(Pointer<Utf8>);
typedef FreeStringFuncNative = Void Function(Pointer<Utf8>);

///////////////////////////////////////////////////////////////////////////////
// Load the library
///////////////////////////////////////////////////////////////////////////////

final DynamicLibrary bidiLib = Platform.isAndroid
    ? DynamicLibrary.open("libbidi.so")
    : DynamicLibrary.process();

///////////////////////////////////////////////////////////////////////////////
// Locate the symbols we want to use
///////////////////////////////////////////////////////////////////////////////

final ReorderFunc reorder = bidiLib
    .lookup<NativeFunction<ReorderFuncNative>>("reorder")
    .asFunction();

final FreeStringFunc freeCString = bidiLib
    .lookup<NativeFunction<FreeStringFuncNative>>("rust_cstr_free")
    .asFunction();

///////////////////////////////////////////////////////////////////////////////
// HANDLERS
///////////////////////////////////////////////////////////////////////////////

String nativeReorder(String name) {
  if (bidiLib == null)
    return "ERROR: The library is not initialized 🙁";

  print("- Bidi bindings found 👍");
  print("  ${bidiLib.toString()}"); // Instance info

  final argName = Utf8.toUtf8(name);
  print("- Calling reorder with argument:  $argName");

  // The actual native call
  final resultPointer = reorder(argName);
  print("- Result pointer:  $resultPointer");

  final reorderedStr = Utf8.fromUtf8(resultPointer);
  print("- Response string:  $reorderedStr");

  // Free the string pointer, as we already have
  // an owned String to return
  print("- Freing the native char*");
  freeCString(resultPointer);

  return reorderedStr;
}

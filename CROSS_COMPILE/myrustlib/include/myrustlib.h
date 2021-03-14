#ifndef MYRUSTLIB_H
#define MYRUSTLIB_H
#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

const char* rust_greeting(const char* to);
void rust_greeting_free(char *);
const char* hello_from_rust(void);


#ifdef __cplusplus
}
#endif
#endif  // MYRUSTLIB_H

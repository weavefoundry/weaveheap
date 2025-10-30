#ifndef WEAVEHEAP_H
#define WEAVEHEAP_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct weaveheap_error {
    int32_t code;
    const char* message;
} weaveheap_error;

void weaveheap_error_clear(weaveheap_error* err);

int32_t weaveheap_version_major(void);
int32_t weaveheap_version_minor(void);
int32_t weaveheap_version_patch(void);

#ifdef __cplusplus
}
#endif

#endif // WEAVEHEAP_H

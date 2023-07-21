// Generated by `wit-bindgen` 0.8.0. DO NOT EDIT!
#ifndef __BINDINGS_HOST_H
#define __BINDINGS_HOST_H
#ifdef __cplusplus
extern "C" {
#endif

#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct {
  char*ptr;
  size_t len;
} host_string_t;

// Imported Functions from `host`
void host_print(host_string_t *msg);

// Exported Functions from `host`
void host_run(void);

// Helper Functions

void host_string_set(host_string_t *ret, const char*s);
void host_string_dup(host_string_t *ret, const char*s);
void host_string_free(host_string_t *ret);

#ifdef __cplusplus
}
#endif
#endif

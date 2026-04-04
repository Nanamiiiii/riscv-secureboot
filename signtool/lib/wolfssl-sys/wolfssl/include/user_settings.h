/* 
 * user_settings.h
 *
 * wolfCrypt user settings.
 *
 */

#ifndef H_USER_SETTINGS_
#define H_USER_SETTINGS_

#include <stdint.h>

#define WOLFSSL_GENERAL_ALIGNMENT 4
#define SINGLE_THREADED
#define WOLFCRYPT_ONLY
#define SIZEOF_LONG_LONG 8
#define WOLFSSL_SP_MATH_ALL
#define WOLFSSL_HAVE_SP_ECC
#define WOLFSSL_SP_384
#define WOLFSSL_SP_521
#define WOLFSSL_HAVE_SP_RSA
#define WOLFSSL_SP_4096
#define TFM_TIMING_RESISTANT
#define HAVE_ECC
#define WOLFSSL_HAVE_SP_ECC
#define ECC_TIMING_RESISTANT
#define HAVE_ECC256
#define HAVE_ECC384
#define HAVE_ECC521
#define HAVE_ED25519
#define HAVE_ED448
#define WOLFSSL_SHAKE256
#define HAVE_RSA
#define WOLFSSL_HAVE_SP_RSA
#define WC_RSA_BLINDING
#define WOLFSSL_KEY_GEN
#define WOLFSSL_SHA512
#define WOLFSSL_SHA384
#define WOLFSSL_SHA3
#undef  NO_SHA256
#define HAVE_CHACHA
#define WOLFSSL_AES_COUNTER
#define WOLFSSL_AES_DIRECT
#define NO_CMAC
#define NO_HMAC
#define NO_RC4
#define NO_SHA
#define NO_DH
#define NO_DSA
#define NO_MD4
#define NO_RABBIT
#define NO_MD5
#define NO_SIG_WRAPPER
#define NO_CERT
#define NO_SESSION_CACHE
#define NO_HC128
#define NO_DES3
#define NO_PWDBASED
#define NO_WRITEV
#define NO_MAIN_DRIVER
#define NO_OLD_RNGNAME
#define NO_WOLFSSL_DIR
#define WOLFSSL_NO_SOCK
#define WOLFSSL_IGNORE_FILE_WARN
#define NO_ERROR_STRINGS
#define BENCH_EMBEDDED
#define NO_CRYPT_TEST
#define NO_CRYPT_BENCHMARK

#endif /* !H_USER_SETTINGS_ */

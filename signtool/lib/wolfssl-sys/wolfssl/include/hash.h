/**
 * @file crypto.h
 * @author Akihiro Saiki (misly.lx00@gmail.com)
 * @brief Cryptography wrapper 
 * @version 0.1
 * @date 2022-12-19
 * 
 * @copyright Copyright (c) 2022
 * 
 */

#ifndef __WOLFSSL_HASH_H__
#define __WOLFSSL_HASH_H__

typedef unsigned char byte;

/* For wolfCrypt SHA3-384 */
#include <wolfssl/wolfcrypt/sha3.h>

/**
 * @brief Allocate sha3 ctx struct
 * 
 * @return wc_Sha3*
 */
wc_Sha3 *sha3_ctx_new();

/**
 * @brief Dispose sha3 ctx struct
 */
void sha3_ctx_free(wc_Sha3 *sha3);

/**
 * @brief Wrapper of wolfCrypt SHA3-384 init
 * 
 * @param sha3 ptr to hash context 
 * @return int 
 */
int sha3_384_init(wc_Sha3 *sha3); 

/**
 * @brief Wrapper of wolfCrypt SHA3-384 update
 * 
 * @param sha3 ptr to hash context 
 * @param data data to hash
 * @param len data length
 * @return int 
 */
int sha3_384_update(wc_Sha3 *sha3, const byte *data, int len); 

/**
 * @brief Wrapper of wolfCrypt SHA3-384 final
 * 
 * @param sha3 ptr to hash context 
 * @param hash buffer to store hash digest
 * @return int 
 */
int sha3_384_final(wc_Sha3 *sha3, byte *hash); 

/**
 * @brief Wrapper of wolfCrypt SHA3-384 free
 * 
 * @param sha3 ptr to hash context 
 * @return int 
 */
void sha3_384_free(wc_Sha3 *sha3);

#endif

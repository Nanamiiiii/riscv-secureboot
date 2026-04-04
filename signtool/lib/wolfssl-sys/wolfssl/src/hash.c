/**
 * @file wolfssl_hash.c
 * @author Akihiro Saiki (misly.lx00@gmail.com)
 * @brief wolfCrypt hash library wrapper 
 * @version 0.1
 * @date 2022-12-19
 * 
 * @copyright Copyright (c) 2022
 * 
 */

#include <stdlib.h>
#include <hash.h>

/**
 * @brief Allocate sha3 ctx struct
 * 
 * @return wc_Sha3*
 */
wc_Sha3 *sha3_ctx_new() {
    wc_Sha3 *sha3 = (wc_Sha3 *) malloc(sizeof(wc_Sha3));
    return sha3;
}

/**
 * @brief Dispose sha3 ctx struct
 * 
 * @return int
 */
void sha3_ctx_free(wc_Sha3 *sha3) {
    if (sha3 == NULL) return;
    free(sha3);
    return;
}

/**
 * @brief Wrapper of wolfCrypt SHA3-384 init
 * 
 * @param sha3 ptr to hash context 
 * @return int 
 */
int sha3_384_init(wc_Sha3 *sha3) {
    return wc_InitSha3_384(sha3, NULL, INVALID_DEVID);
}

/**
 * @brief Wrapper of wolfCrypt SHA3-384 update
 * 
 * @param sha3 ptr to hash context 
 * @param data data to hash
 * @param len data length
 * @return int 
 */
int sha3_384_update(wc_Sha3 *sha3, const byte *data, int len) {
    return wc_Sha3_384_Update(sha3, data, len);
}

/**
 * @brief Wrapper of wolfCrypt SHA3-384 final
 * 
 * @param sha3 ptr to hash context 
 * @param hash buffer to store hash digest
 * @return int 
 */
int sha3_384_final(wc_Sha3 *sha3, byte *hash) {
    return wc_Sha3_384_Final(sha3, hash);
}

/**
 * @brief Wrapper of wolfCrypt SHA3-384 free
 * 
 * @param sha3 ptr to hash context 
 * @return int 
 */
void sha3_384_free(wc_Sha3 *sha3) {
    wc_Sha3_384_Free(sha3);
}

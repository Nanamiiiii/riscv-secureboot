/**
 * @file rng.c
 * @author Akihiro Saiki (misly.lx00@gmail.com)
 * @brief Wrapper of wolfCrypt RNG
 * @version 0.1
 * @date 2023-01-30
 * 
 * @copyright Copyright (c) 2023
 * 
 */

#include <rng.h>

/**
 * @brief Allocate wolfCrypt RNG struct
 * 
 * @return WC_RNG* 
 */
WC_RNG *rng_new() {
    WC_RNG *rng = (WC_RNG *) malloc(sizeof(WC_RNG));
    return rng;
}

/**
 * @brief Init wolfCrypt RNG
 * 
 * @param rng 
 * @return int 
 */
int rng_init(WC_RNG *rng) {
    return wc_InitRng(rng);
}

/**
 * @brief Free wolfCrypt RNG struct
 * 
 * @param rng 
 * @return int 
 */
int rng_free(WC_RNG *rng) {
    int ret = wc_FreeRng(rng);
    if (ret != 0) return ret;
    if (rng != NULL) free(rng);
    return 0;
}

/**
 * @file rng.h
 * @author Akihiro Saiki (misly.lx00@gmail.com)
 * @brief Wrapper of wolfCrypt RNG 
 * @version 0.1
 * @date 2023-01-30
 * 
 * @copyright Copyright (c) 2023
 * 
 */

#include <wolfssl/wolfcrypt/random.h>

/**
 * @brief Allocate wolfCrypt RNG struct
 * 
 * @return WC_RNG* 
 */
WC_RNG *rng_new();

/**
 * @brief Init wolfCrypt RNG
 * 
 * @param rng 
 * @return int 
 */
int rng_init(WC_RNG *rng);

/**
 * @brief Free wolfCrypt RNG struct
 * 
 * @param rng 
 * @return int 
 */
int rng_free(WC_RNG *rng);

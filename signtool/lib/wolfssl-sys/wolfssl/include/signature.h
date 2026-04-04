/**
 * @file wolfssl_signature.h
 * @author Akihiro Saiki (misly.lx00@gmail.com)
 * @brief wolfCrypt signing library wrapper 
 * @version 0.1
 * @date 2023-01-30
 * 
 * @copyright Copyright (c) 2023
 * 
 */

#ifndef __WOLFSSL_SIGNATURE_H__
#define __WOLFSSL_SIGNATURE_H__

typedef unsigned char byte;

#include <wolfssl/wolfcrypt/random.h>

/* ed25519 */
#include <wolfssl/wolfcrypt/ed25519.h>

/**
 * @brief Allocate ed25519 key struct
 * 
 * @return ed25519_key*
 */
ed25519_key *ed25519_new();

/**
 * @brief Wrapper of wolfCrypt ed25519_key init
 * 
 * @param key ptr of ed25519_key
 * @return int
 */
int ed25519_key_init(ed25519_key *key);

/**
 * @brief Wrapper of wolfCrypt ed25519 keypair import
 * 
 * @param privkey 
 * @param privkey_size 
 * @param pubkey 
 * @param pubkey_size 
 * @param key 
 * @return int 
 */
int ed25519_import_key(const byte *privkey, word32 privkey_size,
                       const byte *pubkey, word32 pubkey_size, ed25519_key *key);

/**
 * @brief Wrapper of wolfCrypt ed25519 public key import
 * 
 * @param pubkey 
 * @param pubkey_size 
 * @param key 
 * @return int 
 */
int ed25519_import_public(const byte *pubkey, word32 pubkey_size, ed25519_key *key);

/**
 * @brief Wrapper of wolfCrypt ed25519 message signing
 * 
 * @param msg 
 * @param msg_size 
 * @param signature 
 * @param signature_size 
 * @param key 
 * @return int 
 */
int ed25519_sign_msg(const byte *msg, word32 msg_size, byte *signature, word32 *signature_size, ed25519_key *key);

/**
 * @brief Wrapper of wolfCrypt ed25519 signature verification
 * 
 * @param signature 
 * @param signeture_size 
 * @param msg 
 * @param msg_size 
 * @param result 
 * @param key 
 * @return int 
 */
int ed25519_verify_msg(const byte *signature, word32 signeture_size, 
                       const byte *msg, word32 msg_size, int *result, ed25519_key *key);

/**
 * @brief Wrapper of wolfCrypt ed25519 key generation
 * 
 * @param rng 
 * @param key_size 
 * @param key 
 * @return int 
 */
int ed25519_generate_key(WC_RNG *rng, int key_size, ed25519_key *key);

/**
 * @brief Wrapper of wolfCrypt ed25519 private key export
 * 
 * @param key 
 * @param out 
 * @param size 
 * @return int 
 */
int ed25519_export_private_key(ed25519_key *key, byte *out, word32 *size);

/**
 * @brief Wrapper of wolfCrypt ed25519 public key export
 * 
 * @param key 
 * @param out 
 * @param size 
 * @return int 
 */
int ed25519_export_public_key(ed25519_key *key, byte *out, word32 *size);

/**
 * @brief Wrapper of wolfCrypt ed25519 key free
 * 
 * @param key 
 */
void ed25519_key_free(ed25519_key *key);

/**
 * @brief Deallocate ed25519 key struct
 * 
 * @param key 
 */
void ed25519_free(ed25519_key *key);

#endif

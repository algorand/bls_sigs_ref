#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <assert.h>
#include "bls_c.h"


// Credit: https://stackoverflow.com/questions/7775991/how-to-get-hexdump-of-a-structure-data
void hexDump (const char *desc, const void *addr, const int len);

// very simple and basic tests on bls functions
int test()
{

  char seed[] = "this is a very long seed for BLS tests";
  char rngseed[] = "";
  char msg[] = "this is the message we want bls to sign";

  bls_keys key;
  bls_sig sig;

  int i;
  printf("\nkey generation\n");
  // generate a tuple of keys
  // always remove the last byte of the string so that the inputs
  // matches rust's
  key = c_keygen((uint8_t*)seed, sizeof(seed)-1, 0);

  // dump the output
  hexDump ("pk", key.pk.data, PK_LEN);
  hexDump ("sk", key.sk.data, SK_LEN);

  // sign the message with the key
  printf("\nSigning a message: %s\n", msg);
  sig = c_sign(key.sk, (uint8_t*)msg, sizeof(msg)-1);

  // dump the output
  hexDump ("sig", sig.data, SIG_LEN);

  // verifies the signature
  assert(c_verify(key.pk, (void*)msg, sizeof(msg)-1, sig) == true);

  // release memory of sk
  c_free_sk(key.sk);

  int num_agg =5;
  bls_sig sig_list[num_agg];
  bls_pk pk_list[num_agg];


  for(i=0;i<num_agg;i++){
    printf("the %d-th signature\n", i);

    // use the first 32+i bytes as the seed
    key = c_keygen((void*)seed, 32+i, 0);
    pk_list[i] = key.pk;

    // dump the output
    hexDump ("pk", key.pk.data, PK_LEN);


    // generate the signature list
    sig_list[i] = c_sign(key.sk, (void*)msg, sizeof(msg)-1);

    // dump the output
    hexDump ("sig", sig_list[i].data, SIG_LEN);

    // verifies the signature
    assert(c_verify(key.pk, (void*)msg, sizeof(msg)-1, sig_list[i]) == true);

    // release memory of sk
    c_free_sk(key.sk);
  }

  bls_sig agg_sig =  c_aggregation(sig_list, num_agg);
  hexDump("aggregated signature", agg_sig.data, SIG_LEN);

  // verifies the aggregated signature
  assert(c_verify_agg(pk_list, num_agg, (void*)msg, sizeof(msg)-1, agg_sig) == true);




  return 0;
}

int main(){

  test();
  printf("Hello Algorand\n");
}



// Credit: https://stackoverflow.com/questions/7775991/how-to-get-hexdump-of-a-structure-data
void hexDump (const char *desc, const void *addr, const int len) {
    int i;
    unsigned char buff[17];
    const unsigned char *pc = (const unsigned char*)addr;

    // Output description if given.
    if (desc != NULL)
        printf ("%s:\n", desc);

    if (len == 0) {
        printf("  ZERO LENGTH\n");
        return;
    }
    if (len < 0) {
        printf("  NEGATIVE LENGTH: %i\n",len);
        return;
    }

    // Process every byte in the data.
    for (i = 0; i < len; i++) {
        // Multiple of 16 means new line (with line offset).

        if ((i % 16) == 0) {
            // Just don't print ASCII for the zeroth line.
            if (i != 0)
                printf ("  %s\n", buff);

            // Output the offset.
            printf ("  %04x ", i);
        }

        // Now the hex code for the specific character.
        printf (" %02x", pc[i]);

        // And store a printable ASCII character for later.
        if ((pc[i] < 0x20) || (pc[i] > 0x7e))
            buff[i % 16] = '.';
        else
            buff[i % 16] = pc[i];
        buff[(i % 16) + 1] = '\0';
    }

    // Pad out last line if not exactly 16 characters.
    while ((i % 16) != 0) {
        printf ("   ");
        i++;
    }

    // And print the final ASCII bit.
    printf ("  %s\n", buff);
}

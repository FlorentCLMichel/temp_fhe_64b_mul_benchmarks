#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include "mul_plaintext.h"

#define N_INPUTS 1024
#define FILENAME_LHS "tests/data/mul_data_1_in_lhs.dat"
#define FILENAME_RHS "tests/data/mul_data_1_in_rhs.dat"
#define FILENAME_OUT "tests/data/mul_data_1_out_full.dat"

int main(int argc, char** argv) {

    FILE *file_lhs = NULL;
    FILE *file_rhs = NULL;
    FILE *file_out = NULL;
    uint64_t lhs, rhs;
    uint64_t correct_result_high, correct_result_low;
    size_t read_count;

    // Open files for reading

    file_lhs = fopen(FILENAME_LHS, "r");
    if (file_lhs == NULL) {
        perror("Error opening lhs file");
        return 1;
    }

    file_out = fopen(FILENAME_OUT, "r");
    if (file_out == NULL) {
        perror("Error opening out file");
        fclose(file_lhs);
        fclose(file_rhs);
        return 1;
    }
    
    for (unsigned int index_lhs = 0; index_lhs < N_INPUTS; ++index_lhs) {
        read_count = fscanf(file_lhs, "%lu", &lhs);
        if (read_count != 1) {
            fprintf(stderr, "Error reading lhs from file at index %u\n", index_lhs);
            fclose(file_lhs);
            fclose(file_rhs);
            fclose(file_out);
            return 1;
        }
    
        file_rhs = fopen(FILENAME_RHS, "r");
        if (file_rhs == NULL) {
            perror("Error opening rhs file");
            fclose(file_lhs);
            return 1;
        }

        for (unsigned int index_rhs = 0; index_rhs < N_INPUTS; ++index_rhs) {
            
            read_count = fscanf(file_rhs, "%lu", &rhs);
            if (read_count != 1) {
                fprintf(stderr, "Error reading rhs from file at index %u, %u\n", index_lhs, index_rhs);
                fclose(file_lhs);
                fclose(file_rhs);
                fclose(file_out);
                return 1;
            }

            read_count = fscanf(file_out, "%lu %lu", &correct_result_high, &correct_result_low);
            if (read_count != 2) {
                fprintf(stderr, "Error reading correct_result from file at index %u, %u\n", index_lhs, index_rhs);
                fclose(file_lhs);
                fclose(file_rhs);
                fclose(file_out);
                return 1;
            }
            
            const uint128_t correct_result = (((uint128_t) correct_result_high) << 64) | correct_result_low;

            assert(correct_result == full_64b_mul(lhs, rhs));
        }
       
        fclose(file_rhs);
    }

    // Close files
    fclose(file_lhs);
    fclose(file_out);

    printf("PASSED\n");
    return 0;
}

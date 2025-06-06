#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>
#include "linklist.h"
#include "stringedit.h"
#include "bitwise.h"

typedef uint8_t  U08;
typedef uint16_t U16;
typedef uint32_t U32;

typedef struct _flash_struct
{
    U08 index;
    U32 log;
    U16 data[];
}flash;

int main() {
    U16 arr[] = {1, 2, 3, 4, 5};
    int len = sizeof(arr) / sizeof(U16);

    size_t size = sizeof(flash) + len * sizeof(U16);
    printf("size = %ld\n",size);
    flash* pdata = (flash*)malloc(size);
    U08* encode_arr = (U08*)malloc(size);
    flash* tdata = (flash*)malloc(size);

    pdata->index = len;
    memcpy(pdata->data, arr, len * sizeof(U16));

    
    printf("pdata->index = %u\n", pdata->index);
    printf("pdata->data[1] = %u\n", pdata->data[1]);

    memcpy(encode_arr, pdata, size);
    printf("\n");
    for (int i = 0; i < size; i++) {
        printf("%02d ", encode_arr[i]);
    }
    printf("\n");
    
    encode_arr[0] = 0;
    encode_arr[1] = 1;
    encode_arr[2] = 2;
    encode_arr[3] = 3;

    encode_arr[4] = 4;
    encode_arr[5] = 5;
    encode_arr[6] = 6;
    encode_arr[7] = 7;

    encode_arr[8] = 0;
    encode_arr[9] = 1;

    encode_arr[10] = 1;
    encode_arr[11] = 1;

    memcpy(tdata, encode_arr, size);
    printf("\n");
    for (int i = 0; i < size; i++) {
        printf("%02d ", encode_arr[i]);
    }
    printf("\n\n");
    
    printf("tdata->index = %x\n", tdata->index);
    printf("tdata->log = %x\n", tdata->log);
    printf("tdata->data[0] = %x\n", tdata->data[0]);
    printf("tdata->data[1] = %x\n", tdata->data[1]);

    printf("\nhello world\n");

    free(pdata);
    return 0;
}


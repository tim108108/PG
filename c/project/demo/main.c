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

typedef struct _flash_struct
{
    U08 index;
    U16 data[];
}flash;

int main() {
    U16 arr[] = {1, 2, 3, 4, 5};
    int len = sizeof(arr) / sizeof(U16);

    size_t size = sizeof(flash) + len * sizeof(U16);
    printf("size = %ld\n",size);
    flash* pdata = (flash*)malloc(size);
    U08* encode_arr = (U08*)malloc(size);

    pdata->index = len;
    memcpy(pdata->data, arr, len * sizeof(U16));

    
    printf("pdata->index = %u\n", pdata->index);
    printf("pdata->data[1] = %u\n", pdata->data[1]);

    memcpy(encode_arr, pdata, size);

    for (int i = 0; i < size; i++) {
        printf("%02d ", encode_arr[i]);
    }
    
    printf("\nhello world\n");

    free(pdata);
    return 0;
}


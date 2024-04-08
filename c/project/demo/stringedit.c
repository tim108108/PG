#include <stdio.h>
#include <stdbool.h>

void string_reverse(char *s,int len){
    int i=0,j=len-1;
    while (i<j)
    {
        *(s+i)^=*(s+j);
        *(s+j)^=*(s+i);
        *(s+i)^=*(s+j);
        i++;
        j--;
    }
    
}

void uplowcase(char *ptr,int len,bool up){
    if(up){
        for(int i=0;i<len;i++){
            if(*(ptr+i)<65||*(ptr+i)>122)continue;
            *(ptr+i)&=~(1<<5);
        }
    }
    else{
        for(int i=0;i<len;i++){
            *(ptr+i)|=(1<<5);
        }
    }
}
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

void string_reverse(char *ptr,int len){
    int i=0,j=len-1;
    while(i<j){
    *(ptr+i)^=*(ptr+j);
    *(ptr+j)^=*(ptr+i);
    *(ptr+i)^=*(ptr+j);
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
int setbit(int a,int pos,bool b){
    if(b){
        return (a|(1<<pos));
    }
    return (a&~(1<<pos));
}

void point(){
    int a[]={0,2,4,6,8};
    int *ptr=a;
    int b[5]={1,2,3,4,5};
    int *p1=b;
    int *p2=b;
    printf("{1,2,3,4,5}\n");
    printf("print:*(b++)=%d\n",*(p1++));
    printf("print:*(++b)=%d\n",*(++p2));

    printf("*(ptr+0):%d\n",*ptr);
    printf("*(ptr+1):%d\n",*(ptr+1));
    // printf("*(ptr+2):%d\n",*(ptr+2));
    // printf("*(ptr+3):%d\n",*(ptr+3));
    printf("*(ptr+4):%d\n",*(ptr+4));//指向int的指標移動4個int(16byte)
    printf("*ptr+10:%d\n",*ptr+10);//指標的值(0)+10=10
    printf("ptr+0:%p\n",ptr);
    printf("ptr+1:%p\n",ptr+1);
    // printf("ptr+2:%p\n",ptr+2);
    // printf("ptr+3:%p\n",ptr+3);
    printf("ptr+4:%p\n",ptr+4);
    printf("ptr+5:%p\n",ptr+5);
    printf("&a+0     :%p\n",&a);//陣列的頭(&a[0])的位址
    printf("&a+1     :%p\n",&a+1);//陣列的頭(&a[0])增加一個a陣列長度的位址
    printf("&(a[0])+1:%p\n",&(a[0])+1);//陣列
    printf("&(a)+1   :%p\n",&(a)+1);
}
double Pow(int a,int b){
    double tmp=1;
    for(int i=0;i<b;i++){
        tmp*=a;
    }
    return tmp;
}
double leibniz(int n){
    double l=0;
    for(int k=1;k<=n;k++){
        l=l+Pow(-1.0,k-1)/(2*k-1);
    }
    return 4*l;
}

void swap(int *a,int *b){
    int tmp;
    tmp=*a;
    *a=*b;
    *b=tmp;
}

int Partition(int *arr, int front, int end){
    int pivot = arr[end];
    int i = front -1;
    for (int j = front; j < end; j++) {
        if (arr[j] < pivot) {
            i++;
            swap(&arr[i], &arr[j]);
        }
    }
    i++;
    swap(&arr[i], &arr[end]);
    return i;
}
void QuickSort(int *arr, int front, int end){
    if (front < end) {
        int pivot = Partition(arr, front, end);
        QuickSort(arr, front, pivot - 1);
        QuickSort(arr, pivot + 1, end);
    }
}

int binsearch(int *arr,int key,int n){
    int left=0,right=n;
    while(left<=right){
        int mid=(left+right)/2;
        if(arr[mid]>key){
            right=mid-1;
        }
        else if(arr[mid]<key){
            left=mid+1;
        }
        else{
            return mid;
        }
    }
    return -1;
}
void Q1(int *in, int size){
    int *ptr = in;
    for(int i=0;i<size;i++){
        if (*ptr == i){
            ptr++;
        }
        else{
            printf("%d\n",i);

        }
    }
    
}

void Q2(int *a,int size,int b){
    int *ptr=a;
    
    while(*ptr<b*100){
        ptr++;
    }
    
    for(int i=0;i<(b+1)*100;i++){
        if(*ptr==i){
            ptr++;
        }
        else{
            printf("%d\n",i);
        }
    }
}
void sizeQ(){
    char *s = "hello"; 
    char s1[]={'h','e','l','l','o'}; 
    int s2[]={'h','e','l','l','o'}; 
    printf("s=%ld,s1=%ld,s2=%ld\n",sizeof(s),sizeof(s1),sizeof(s2));//8,5,5*4
}

unsigned long long fib(int a){
    int f0=0,f1=1,fn;
    if(a==0){
        return f0;
    }
    if(a==1)
        return f1;
    for(int i=2;i<=a;i++){
        fn = f0+f1;
        f0=f1;
        f1=fn;
    }
    return fn;
}

void diamond(int a){
    if(a%2){
        int z=a/2,x=1;
        for(int i=0;i<(a+1)/2;i++){
            for(int j=0;j<z;j++){
                printf(" ");
            }
            for(int k=0;k<x;k++){
                printf("*");
            }
            x+=2;
            z--;
            printf("\n");
        }
        z=a-2,x=1;
        for(int i=0;i<a/2;i++){
            for(int j=0;j<x;j++){
                printf(" ");
            }
            for(int k=0;k<z;k++){
                printf("*");
            }
            x++;
            z-=2;
            printf("\n");
        }
    }
    else
        diamond(a+3);
}

int GCD(int a,int b){
    int r;
    r=a%b;
    if(r==0)
        return b;
    else 
        return GCD(b,r);
}

void bubble_sort(int *arr,int len){
    for(int i=0;i<len;i++){
        for(int j=0;j<i;j++){
            if(arr[i]<arr[j]){
                //printf("i:%d,j:%d\n",i,j);
                int tmp=arr[i];
                arr[i]=arr[j];
                arr[j]=tmp;
            }
        }
    }
}

void insertion_sort(int *arr,int len){
    
}

int main(){
    // sizeQ();
    // for(int i=0;i<40;i++){
    //     printf("%llu\n",fib(i));
    // }
    int a[]={1,2,3,4,13,5,9,8};
    bubble_sort(a,8);
    for (int i=0;i<8;i++){
    printf("%d,",a[i]);
    }
    printf("\n");
}
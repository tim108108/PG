#include <stdio.h>
#include <stdlib.h>
void bubble_sort(int *arr, int size);/*氣泡排序法*/
void select_sort(int *arr, int size);/*選擇排序法*/

void bubble_sort(int *arr, int size)
{
    int i,j;
    for(i=0;i<size;i++){
	    for(j=0;j<i;j++){
	        if(*(arr+i)<*(arr+j)){
                *(arr+i)^=*(arr+j);
                *(arr+j)^=*(arr+i);
                *(arr+i)^=*(arr+j);
		    }
	    }
    }
}
select_sort(int *arr, int size)
{
    int i,j;
}

int main()
{
    int arr[] = {1,3,21,5,14,2,12,13,23,43,44,41,2,2};
    int i;
    bubble_sort(arr,sizeof(arr)/sizeof(int));
    for(i=0;i<(sizeof(arr)/sizeof(int));i++){
        printf("%d,",arr[i]);
    }
    printf("\n");
}

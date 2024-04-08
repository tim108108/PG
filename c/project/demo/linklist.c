#include <stdio.h>
#include <stdlib.h>
#include "linklist.h"

NODE *creatList(int *arr,int len){
    int i;
    NODE *first,*current,*previuos;
    for(i=0;i<len;i++){
        current=(NODE *) malloc(sizeof(NODE));
        current->data=arr[i];
        if(i==0){
            first=current;
        }
        else{
            previuos->next=current;
        }
        current->next=NULL;
        previuos=current;
    }
    return first;
}

void printList(NODE *first){
    NODE *ptr=first;
    if(first==NULL){
        printf("error");
    }
    while (ptr!=NULL)
    {
        printf("%d ",ptr->data);
        ptr=ptr->next;
    }
    printf("\n");
}

void freeList(NODE *first){
    NODE *tmp,*ptr=first;
    while(ptr!=NULL){
        tmp=ptr;
        ptr=ptr->next;
        free(tmp);
    }
}

NODE *searchNode(NODE *ll,int item){
    while (ll!=NULL)
    {   
        if(ll->data==item){
            return ll;
        }
        else{
            ll=ll->next;
        }
    }
    return NULL;
    
}

void insertNode(NODE *node,int item){
    NODE *newnode;
    newnode=(NODE *)malloc(sizeof(NODE));
    newnode->data=item;
    newnode->next=node->next;
    node->next=newnode;
}

NODE *deleteNode(NODE *first,NODE *node){
    NODE *tmp=first;
    if(first==NULL){
        printf("Noting to delete.\n");
        return NULL;
    }
    if(node==first){
        first=first->next;
    }
    else{
        while (tmp->next!=node)
        {
            tmp=tmp->next;
        }
        tmp->next=node->next;
    }
    free(node);
    return first;
}

NODE *Qpush(NODE *first,int v){
    NODE *q;
    q = (NODE*) malloc(sizeof(NODE));
    q->data=first->data;
    first->data=v;
    q->next=first->next;
    first->next=q;
    return first;
}

NODE *Qpop(NODE *first){
    NODE *ptr,*end=first;
    while (end->next){
        ptr=end;
        end=end->next;
    }
    ptr->next=NULL;
    return end;
}
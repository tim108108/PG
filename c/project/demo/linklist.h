typedef struct node
{
    int data;
    struct node *next;
}NODE;

typedef struct queue //FIFO
{
    NODE *front;
    NODE *rear;
}QUE;

NODE *creatList(int *,int);
void printList(NODE *);
void freeList(NODE *);
NODE *searchNode(NODE *,int);
void insertNode(NODE *,int);
NODE *deleteNode(NODE *,NODE *);
NODE *Qpush(NODE *,int);
NODE *Qpop(NODE *);
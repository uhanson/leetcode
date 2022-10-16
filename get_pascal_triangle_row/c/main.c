#include <stdio.h>
#include <stdlib.h>

void print_row(int *row, int rowSize) {
    for (int i = 0; i < rowSize; i++) {
        printf("%d", row[i]);

        if (i < rowSize - 1) {
            printf(",");
        }
    }

    printf("\n");
}

int* getRow(int rowIndex, int* returnSize){
    int *row = (int*) malloc(rowIndex + 1);

    if (rowIndex > 0) {
        int *lastRow = getRow(rowIndex - 1, returnSize);

        row[0] = 1;
        row[rowIndex] = 1;

        for (int i = 1; i < rowIndex; i++) {
            row[i] = lastRow[i - 1] + lastRow[i];
        }

        free(lastRow);
    } else {
        row[0] = 1;
    }

    return row;
}

void main() {
    int returnSize = 0;

    free(getRow(3, &returnSize));
}
#include <iostream>
#include <vector>
#include <stdio.h>

void print(std::vector<int>& a){
    printf("Vector: ");
    for (const auto& item : a){
        printf("%d ", item);
    }
    printf("\n_");
}

int main(){
    std::vector<int> a;
    std::vector<int> b = a;
    std::vector<int> c = b;

    b.push_back(1);
    c.push_back(1);
    printf("a");
    print(a);
    printf("b");
    print(b); 
    printf("c");
    print(c);
}
package main

import "fmt"

// type GoEnum = int
// const(
// 	Foo GoEnum = iota
// 	Bar
// 	Baz
// )

func returnsError(value int) error {
	return fmt.Errorf("this is an error with value %v", value)
}

type Foo struct{

}

func (f *Foo) thisIsOnFoo(value int) error {
	return fmt.Errorf("this is an error with value %v", value)
}

func CreateFoo(fail bool)(*Foo, error){
	if fail {
		return nil, fmt.Errorf("this is an error")
	}
	return &Foo{}, nil
} 

func main(){
	foo, err := CreateFoo(false)
	if err!=nil {
		fmt.Println(err)
	}
	fmt.Println(foo)
}
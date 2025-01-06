package main

/*
#cgo LDFLAGS: -L. -lrust_lib
#include <stdint.h>
#include <stdlib.h>

struct Person {
    char* name;
    int age;
};

struct Error {
    int error_code;
    char* message;
};

extern struct Person* create_person(const char* name, int age);
extern struct Person* get_person_details(struct Person* person);
extern void free_person(struct Person* person);
extern struct Error* get_error_message();
*/
import "C"
import (
	"fmt"
	"time"
	"unsafe"
)

func main() {

	startTime := time.Now()
	count := 100
	for i := 0; i < count; i++ {
		// 创建一个Person
		name := C.CString("Alice")
		defer C.free(unsafe.Pointer(name))

		person := C.create_person(name, 30)
		defer C.free_person(person)

		if person == nil {
			fmt.Println("Failed to create person")
			return
		}

		// 获取并修改Person
		updatedPerson := C.get_person_details(person)
		if updatedPerson != nil {
			defer C.free_person(updatedPerson)
			fmt.Printf("Updated person: %s, Age: %d\n", C.GoString(updatedPerson.name), updatedPerson.age)
		} else {
			fmt.Println("Error: Invalid person or age less than 18")
		}

		// 错误处理示例
		errorMessage := C.get_error_message()
		defer func() {
			if errorMessage != nil {
				C.free(unsafe.Pointer(errorMessage.message))
				C.free(unsafe.Pointer(errorMessage))
			}
		}()
		if errorMessage != nil {
			fmt.Printf("Error code: %d, Message: %s\n", errorMessage.error_code, C.GoString(errorMessage.message))
		}
	}
	// 耗时：1.646162ms
	fmt.Printf("耗时：%s\n", time.Since(startTime).String())
	
}

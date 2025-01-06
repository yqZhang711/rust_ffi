# Go如何调用C

## 基本用法
需要在 Go 代码中导入 C 包，并使用注释来包含 C 代码。CGo 会将这些注释中的代码视为 C 代码，并在编译时将其与 Go 代码一起编译。
```
package main

/*
#include <stdio.h>

void sayHello() {
    printf("Hello from C!\n");
}
*/
import "C"

func main() {
    C.sayHello()
}
```

## 参数传递
在 Go 和 C 之间传递参数和返回值。CGo 会自动处理 Go 和 C 之间的类型转换。

```
package main

/*
#include <stdio.h>

int add(int a, int b) {
    return a + b;
}
*/
import "C"
import "fmt"

func main() {
    a := C.int(3)
    b := C.int(4)
    result := C.add(a, b)
    fmt.Println("Result:", result)
}
```

## 处理复杂类型
对于复杂类型（如结构体），需要在 Go 和 C 之间进行手动转换。
```
package main

/*
#include <stdio.h>

typedef struct {
    int x;
    int y;
} Point;

void print_point(Point p) {
	printf("Point(%d, %d)\n", p.x, p.y);
}
*/
import "C"

func main() {
	//goPoint := struct {
	//	x, y int
	//}{x: 10, y: 20}
	//
	//// 将 Go 结构体转换为 C 结构体
	//cPoint := C.struct_Point{
	//	x: C.int(goPoint.x),
	//	y: C.int(goPoint.y),
	//}
	//
	//// 调用 C 函数
	//C.print_point(cPoint)

	p := C.Point{x: 10, y: 20}
	C.print_point(p)
}
```
可参考[《Co语言高级编程》](https://chai2010.cn/advanced-go-programming-book/ch2-cgo/ch2-03-cgo-types.html)中的一节


## 如何将C编译成共享库(动态库)
```
// Linux/macOS 上
gcc -shared -o libmylib.so -fPIC mylib.c 

//  Windows 上
gcc -shared -o mylib.dll -Wl,--out-implib,libmylib.a mylib.c 
```
编译后会生成一个共享库文件（如 libmylib.so 或 mylib.dll）。

### golang 如何调用
```
package main

/*
#cgo LDFLAGS: -L. -lmylib
#include <stdlib.h>

// 声明 C 函数
void sayHello();
int add(int a, int b);
*/
import "C"
import (
	"fmt"
)

func main() {
	// 调用 C 函数
	C.sayHello()

	// 调用带参数的 C 函数
	result := C.add(C.int(3), C.int(4))
	fmt.Println("Result of add(3, 4):", result)
}

```
关键点：
   - cgo LDFLAGS：指定链接器的标志。
   - -L. 表示在当前目录查找共享库。
      - -L. 表示在当前目录查找库文件。
      - -L/path/to/libs 表示在 /path/to/libs 目录查找库文件。
   - -lmylib 表示链接名为 libmylib.so 或 libmylib.dll 的库
   - -l 参数的作用 （为什么是-lmylib，而不是-libmylib.so）-l参数用于指定要链接的库文件。它的规则是：
      - 去掉库文件名中的前缀 lib 和后缀（如 .so 或 .a），例如，库文件名为 libmylib.so，使用 -l 参数时只需写 -lmylib
      - GCC 链接器会自动为 -l 参数添加前缀 lib 和后缀（如 .so 或 .a）。因此：当你写 -lmylib 时，链接器会查找名为 libmylib.so 或 libmylib.a 的文件。如果你写 -libmylib，链接器会尝试查找名为 liblibmylib.so 或 liblibmylib.a 的文件，这就找不到文件了。
  
## 如何将C编译成静态库
```
# Linux/macOS 上
gcc -c mylib.c -o mylib.o        # 编译为目标文件
ar rcs libmylib.a mylib.o        # 打包为静态库

#Windows 上：
gcc -c mylib.c -o mylib.o        # 编译为目标文件
ar rcs libmylib.a mylib.o        # 打包为静态库
```
编译后会生成一个静态库文件 libmylib.a

### Go 中调用静态库
package main

/*
#cgo LDFLAGS: -L. -lmylib
#include <stdlib.h>

// 声明 C 函数
void sayHello();
int add(int a, int b);
*/
import "C"
import "fmt"

func main() {
    // 调用 C 函数
    C.sayHello()

    // 调用带参数的 C 函数
    result := C.add(C.int(3), C.int(4))
    fmt.Println("Result of add(3, 4):", result)
}

## 动态库和静态库优缺点
   说实话没搞太清楚， 
   可参考[《Co语言高级编程》](https://chai2010.cn/advanced-go-programming-book/ch2-cgo/ch2-09-static-shared-lib.html)中的一节


## 动态库和静态库的命名规则
无论是动态库还是静态库，GCC 都遵循相同的命名规则：
 - 动态库：lib<name>.so（Linux），lib<name>.dylib（macOS），<name>.dll（Windows）
 - 静态库：lib<name>.a（Linux/macOS），<name>.lib（Windows）。


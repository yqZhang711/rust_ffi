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
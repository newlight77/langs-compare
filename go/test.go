package main

import ("fmt"
        "time"
)

func main() {
    start := time.Now()
        ch := make(chan int)
	for k := 0; k <= 10; k++ {
		go term(ch, int(k))
	}
	f := 0
	for k := 0; k <= 10; k++ {
		f += <-ch
	}
	
    fmt.Println(time.Since(start))
}

func term(ch chan int, k int) {
         for k := 0; k <= 5000000; k++ {
           ch <- (k+1) 
        }
}

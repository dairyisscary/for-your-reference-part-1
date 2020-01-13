package main

import (
    "strconv"
    "fmt"
    "io/ioutil"
    "math/rand"
    "os"
    "time"
)

type accumlator struct {
    sum int64
}

func accumulate(accum *accumlator, filename string, done chan bool) {
    file, err := os.Open(filename)
    if err != nil {
        panic(err)
    }
    defer file.Close()

    bytes, err := ioutil.ReadAll(file)
    if err != nil {
        panic(err)
    }
    contents := string(bytes[:len(bytes) -1])
    num, err := strconv.ParseInt(contents, 10, 32)
    if err != nil {
        panic(err)
    }

    time.Sleep(time.Duration(rand.Intn(20)) * time.Millisecond)
    cur := accum.sum
    time.Sleep(time.Duration(rand.Intn(20)) * time.Millisecond)
    accum.sum = num + cur
    done <- true
}

func main() {
    filenames := os.Args[1:]
    accum := accumlator{sum: 0}
    done := make(chan bool, len(filenames))
    for _, filename := range filenames {
        go accumulate(&accum, filename, done)
    }
    for _, _ = range filenames {
        <- done
    }
    fmt.Printf("The sum of the files is %d.\n", accum.sum)
}

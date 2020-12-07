package main

import (
    "fmt"
    "log"
    "os"
)

func panic_if(e error) {
	if e != nil {
		log.Fatal(e)
		panic(e)
	}
}

func seat_id(row int, col int) int {
	return row * 8 + col
}

type data_compiler struct {
    max   int
    seats [128 * 8]bool
}

func (c *data_compiler) push(row int, col int) {
	seat := seat_id(row, col)
	if seat > c.max {
		c.max = seat
	}

	c.seats[seat] = true
}

type digest struct {
	max     int
	my_seat int
}

func (c *data_compiler) digest() digest {
	len := len(c.seats)

	for i := 1; i < len; i++ {
		if c.seats[i-1] && !c.seats[i] {
			return digest{max: c.max, my_seat: i}
		}
	}

	panic("unreachable in normal conditions")
}

func main() {
	f, err := os.Open("input.txt")
    panic_if(err)
	defer f.Close()

	data := data_compiler{}

	for {
		buf := [11]byte{}
		buf_len, err := f.Read(buf[:])
		panic_if(err)

		if buf_len < len(buf) {
			break
		}

		var row, col int

		power := 64
		for i := 0; i < 7; i++ {
			if buf[i] == 'B' {
				row += power
			}
			power /= 2
		}

		power = 4
		for i := 7; i < 11; i++ {
			if buf[i] == 'R' {
				col += power
			}
			power /= 2
		}

		data.push(row, col)
	}

	fmt.Printf("%+v\n", data.digest())
}

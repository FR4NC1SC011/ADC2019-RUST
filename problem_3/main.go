package main

import (
	"io/ioutil"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

type Coordinates struct {
	x int
	y int
}

var wire1 []Coordinates
var wire2 []Coordinates

func main() {
	file, err := ioutil.ReadFile(os.Args[1])
	check(err)

	input := string(file)
	wires := strings.Split(input, "\n")
	wire1 = wire_path(wires[0])
	wire2 = wire_path(wires[1])

	intersect_points := intersection(wire1, wire2)
	// fmt.Println(intersect_points)

	findMinimumDistance(intersect_points, Coordinates{0, 0})

}

func wire_path(wire string) []Coordinates {
	x := 0
	y := 0

	var path []Coordinates

	w := strings.Split(wire, ",")
	for _, i := range w {
		dir := string(i[0])
		c, _ := strconv.Atoi(i[1:])

		switch dir {
		case "R":
			for j := 0; j < c; j++ {
				x++
				path = append(path, Coordinates{x, y})
			}
		case "L":
			for j := 0; j < c; j++ {
				x--
				path = append(path, Coordinates{x, y})
			}
		case "U":
			for j := 0; j < c; j++ {
				y++
				path = append(path, Coordinates{x, y})
			}
		case "D":
			for j := 0; j < c; j++ {
				y--
				path = append(path, Coordinates{x, y})
			}
		}
	}
	return path
}

func intersection(wire1 []Coordinates, wire2 []Coordinates) []Coordinates {
	var intersect_points []Coordinates

	for _, v := range wire1 {
		for _, w := range wire2 {
			if v.x == w.x && v.y == w.y {
				intersect_points = append(intersect_points, v)
			}
		}
	}

	return intersect_points
}

func findMinimumDistance(cuts []Coordinates, origin Coordinates) {
	min := math.MaxInt64
	for _, cut := range cuts {
		mh := computeDistance(origin, cut)
		if mh < min {
			min = mh
		}
		log.Printf("%v -> %d\n", cut, mh)
	}
	log.Printf("Minimum distance %d\n", min)
}

func computeDistance(origin Coordinates, cut Coordinates) int {
	return int(math.Abs(float64(origin.x-cut.x)) + math.Abs(float64(origin.y-cut.y)))
}

func check(e error) {
	if e != nil {
		log.Fatal("Error: ", e)
	}
}

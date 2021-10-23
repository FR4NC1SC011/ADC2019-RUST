package main

import (
	"log"
)

var initial_int int = 231832

// var starting_range = []int{2, 3, 1, 8, 3, 2}

var final_int = 767346

// var final_range = []int{7, 6, 7, 3, 4, 6}

var password_count int = 0

// var password_length int = 5 //012345

func main() {
	solve_A()
}

func solve_A() {
	for i := initial_int; i <= final_int; i++ {
		var y = []int{}
		x := int_to_slice(i, y)

		if first_rule(x) {
			if second_rule(x) {
				log.Println(x)
				password_count++
			}
		}
	}

	log.Println("Total:", password_count)
}

// Two adjacent digits are the same (like 22 in 122345).
func first_rule(password []int) bool {

	for i, d := range password {
		if i == 5 {
			break
		}

		if d == password[i+1] {
			return true
		}
	}
	return false
}

// Going from left to right, the digits never decrease;
func second_rule(password []int) bool {

	for i := 0; i < len(password)-1; i++ {
		if password[i] > password[i+1] {
			return false
		}
	}

	return true
}

func int_to_slice(n int, sequence []int) []int {
	if n != 0 {
		i := n % 10
		sequence = append([]int{i}, sequence...)
		return int_to_slice(n/10, sequence)
	}
	return sequence
}

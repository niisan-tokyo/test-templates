package fizzbuzz

import "testing"

func Test_1をいれたら1が返る(t *testing.T) {
	if fizzbuzz(1) != "1" {
		t.Error(`miss`)
	}
}

func Test_2をいれたら2が返る(t *testing.T) {
	if fizzbuzz(2) != "2" {
		t.Error(`miss`)
	}
}

func Test_3をいれたらFizzが返る(t *testing.T) {
	if fizzbuzz(3) != "Fizz" {
		t.Error(`miss`)
	}
}

func Test_6をいれたら6が返る(t *testing.T) {
	if fizzbuzz(6) != "Fizz" {
		t.Error(`miss`)
	}
}

func Test_5をいれたらBuzzが返る(t *testing.T) {
	if fizzbuzz(5) != "Buzz" {
		t.Error(`miss`)
	}
}

func Test_10をいれたらBuzzが返る(t *testing.T) {
	if fizzbuzz(10) != "Buzz" {
		t.Error(`miss`)
	}
}

func Test_15をいれたらFizzBuzzが返る(t *testing.T) {
	if fizzbuzz(15) != "FizzBuzz" {
		t.Error(`miss`)
	}
}

func Test_30をいれたらFizzBuzzが返る(t *testing.T) {
	if fizzbuzz(30) != "FizzBuzz" {
		t.Error(`miss`)
	}
}

<?php
namespace App;

class FizzBuzz
{
    public function run(int $num): string
    {
        if ($num % 3 === 0) {
            return 'Fizz';
        }

        if ($num % 5 === 0) {
            return 'Buzz';
        }
        return (string)$num;
    }
}
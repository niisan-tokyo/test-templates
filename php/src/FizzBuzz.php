<?php
namespace App;

class FizzBuzz
{
    public function run(int $num): string
    {
        if ($num % 3 === 0) {
            return 'Fizz';
        }
        return (string)$num;
    }
}
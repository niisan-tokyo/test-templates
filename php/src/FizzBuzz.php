<?php
namespace App;

class FizzBuzz
{
    public function run(int $num): string
    {
        $str = ($num % 3 === 0) ? 'Fizz': '';
        $str .= ($num % 5 === 0) ? 'Buzz': '';
        return ($str) ? $str: (string)$num;
    }
}
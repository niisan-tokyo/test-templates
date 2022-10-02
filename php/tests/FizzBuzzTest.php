<?php
namespace Tests;

use App\FizzBuzz;
use PHPUnit\Framework\TestCase;

class FizzBuzzTest extends TestCase
{

    private FizzBuzz $obj;

    public function setUp(): void
    {
        $this->obj = new FizzBuzz;
    }

    /**
     * @test
     */
    public function 値に1をいれたら1を返す()
    {
        $this->assertEquals('1', $this->obj->run(1));
    }

    /**
     * @test
     */
    public function 値に2を入れたら2を返す()
    {
        $this->assertEquals('2', $this->obj->run(2));
    }

    /**
     * @test
     */
    public function 値に3をいれたらFizzを返す()
    {
        $this->assertEquals('Fizz', $this->obj->run(3));
    }

    /**
     * @test
     */
    public function 値に6をいれたらFizzを返す()
    {

    }

    /**
     * @test
     */
    public function 値に5をいれたらBuzzを返す()
    {

    }

    /**
     * @test
     */
    public function 値に10をいれたらBuzzを返す()
    {

    }

    /**
     * @test
     */
    public function 値に15をいれたらFizzBuzzを返す()
    {

    }

    /**
     * @test
     */
    public function 値に30をいれたらFizzBuzzを返す()
    {

    }
}
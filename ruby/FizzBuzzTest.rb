require 'test/unit'
require './FizzBuzz'

class FizzBuzzTest < Test::Unit::TestCase
    def setup 
        @obj = FizzBuzz.new
    end

    def test_1を入れたら1が返る
        assert_equal 1, @obj.run(1)
    end
    def test_2を入れたら2が返る
        assert_equal 2, @obj.run(2)
    end
    def test_3を入れたらFizzが返る
        assert_equal 'Fizz', @obj.run(3)
    end
    def test_6を入れたらFizzが返る
        assert_equal 'Fizz', @obj.run(6)
    end
    def test_5を入れたらBuzzが返る

    end
    def test_10を入れたらBuzzが返る

    end
    def test_15を入れたらFizzBuzzが返る

    end
    def test_30を入れたらFizzBuzzが返る

    end
end
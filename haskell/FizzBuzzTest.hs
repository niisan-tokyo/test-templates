import Test.HUnit
import FizzBuzz 

tests = [
        "1を入れたら'1'を返す" ~: fizzbuzz 1 ~?= "1",
        "2を入れたら'2'を返す" ~: fizzbuzz 2 ~?= "2",
        "3を入れたら'Fizz'を返す" ~: fizzbuzz 3 ~?= "Fizz",
        "6を入れたら'Fizz'を返す" ~: fizzbuzz 6 ~?= "Fizz",
        "5を入れたら'Buzz'を返す" ~: fizzbuzz 5 ~?= "Buzz",
        "10を入れたら'Buzz'を返す" ~: fizzbuzz 10 ~?= "Buzz",
        "15を入れたら'FizzBuzz'を返す" ~: fizzbuzz 15 ~?= "FizzBuzz",
        "30を入れたら'FizzBuzz'を返す" ~: fizzbuzz 30 ~?= "FizzBuzz"
    ]

main::IO ()
main = do
    counts <- runTestTT $ TestList tests
    return ()
class FizzBuzz
    def run(num)
        if num.modulo(3) === 0
            return 'Fizz'
        end

        if num.modulo(5) === 0
            return 'Buzz'
        end
        num
    end
end
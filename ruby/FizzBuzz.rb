class FizzBuzz
    def run(num)
        str = ''
        str += 'Fizz' if num % 3 === 0
        str += 'Buzz' if num % 5 === 0
        str.empty? ? num.to_s : str
    end
end
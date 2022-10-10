class FizzBuzz:
    def run(self, num):
        if num % 3 == 0:
            return "Fizz"

        if num % 5 == 0:
            return "Buzz"
        return str(num)

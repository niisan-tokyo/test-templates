class FizzBuzz:
    def run(self, num):
        st = ""
        st += "Fizz" if num % 3 == 0 else ""
        st += "Buzz" if num % 5 == 0 else ""
        return str(num) if st == "" else st

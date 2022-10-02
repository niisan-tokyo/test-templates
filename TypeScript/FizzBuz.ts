export class FizzBuzz {
    public run(num: number): string {
        if (num % 3 === 0) {
            return 'Fizz'
        }
        return num.toString()
    }
}
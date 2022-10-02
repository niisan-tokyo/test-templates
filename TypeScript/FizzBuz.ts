export class FizzBuzz {
    public run(num: number): string {
        let str = ''
        str += num % 3 === 0 ? 'Fizz' : ''
        str += num % 5 === 0 ? 'Buzz' : ''
        return str !== '' ? str : num.toString()
    }
}
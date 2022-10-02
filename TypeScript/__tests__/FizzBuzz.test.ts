import { FizzBuzz } from '../FizzBuz'

const obj = new FizzBuzz()

describe('FizzBuzzのテスト', () => {
    test('1をいれたら1を返す', () => {
        expect(obj.run(1)).toBe('1')
    })

    test('2をいれたら2を返す', () => {
        expect(obj.run(2)).toBe('2')
    })

    test('3をいれたらFizzを返す', () => {
        expect(obj.run(3)).toBe('Fizz')
    })

    test('6をいれたらFizzを返す', () => {
        expect(obj.run(6)).toBe('Fizz')
    })

    test('5をいれたらBuzzを返す', () => {
        
    })

    test('10をいれたらBuzzを返す', () => {
        
    })

    test('15をいれたらFizzBuzzを返す', () => {
        
    })

    test('30をいれたらFizzBuzzを返す', () => {
        
    })
})
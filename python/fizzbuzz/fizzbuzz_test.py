import unittest
from fizzbuzz import FizzBuzz

class TestFizzBuzz(unittest.TestCase):
    def setUp(self):
        self.obj = FizzBuzz()

    def test_1をいれたら1を返す(self):
        self.assertEqual(self.obj.run(1), "1")

    def test_7をいれたら7を返す(self):
        self.assertEqual(self.obj.run(7), "7")

    def test_3をいれたらFizzを返す(self):
        self.assertEqual(self.obj.run(3), "Fizz")

    def test_9をいれたらFizzを返す(self):
        self.assertEqual(self.obj.run(9), "Fizz")

    def test_5をいれたらBuzzを返す(self):
        self.assertEqual(self.obj.run(5), "Buzz")

    def test_20をいれたらBuzzを返す(self):
        self.assertEqual(self.obj.run(20), "Buzz")

    def test_15をいれたらFizzBuzzを返す(self):
        self.assertEqual(self.obj.run(15), "FizzBuzz")

    def test_45をいれたらFizzBuzzを返す(self):
        None#

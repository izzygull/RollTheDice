from random import seed
import unittest
import roll_parser


class TestParensBalance(unittest.TestCase):
    def testEmpty(self):
        self.assertEqual(roll_parser.parse_parens(''), True)

    def testTrueMin(self):
        self.assertEqual(roll_parser.parse_parens('()'), True)

    def testTrueWithContents(self):
        self.assertEqual(roll_parser.parse_parens('(Hello)'), True)

    def testTrueMultiple(self):
        self.assertEqual(roll_parser.parse_parens(
            '(Hello (I am Here) Goodbye)'), True)

    def testFalseOpen(self):
        self.assertEqual(roll_parser.parse_parens('(()'), False)

    def testFalseClose(self):
        self.assertEqual(roll_parser.parse_parens('())'), False)

    def testFalseWrongOrder(self):
        self.assertEqual(roll_parser.parse_parens(')('), False)


class TestCheckMatch(unittest.TestCase):
    def testEmpty(self):
        self.assertEqual(roll_parser.check_match(''), False)

    def testSingleNum(self):
        self.assertEqual(roll_parser.check_match('6'), True)

    def testSingleDie(self):
        self.assertEqual(roll_parser.check_match('d6'), True)

    def testMultipliedDie(self):
        self.assertEqual(roll_parser.check_match('6d6'), True)

    def testMultipleDice(self):
        self.assertEqual(roll_parser.check_match('4d4+6d6'), True)

    def testMultipleDigits(self):
        self.assertEqual(roll_parser.check_match(
            '1234567890d9876543210'), True)

    def testAllOperators(self):
        self.assertEqual(roll_parser.check_match('5+4-3*2/1'), True)

    def testParensSingleExpr(self):
        self.assertEqual(roll_parser.check_match('(4d4)'), True)

    def testParensMultiExpr(self):
        self.assertEqual(roll_parser.check_match('4d4*(2d6+98)'), True)

    def testOperatorAlone(self):
        self.assertEqual(roll_parser.check_match('+'), False)

    def testInitialOperator(self):
        self.assertEqual(roll_parser.check_match('+d4'), False)

    def testHangingOperator(self):
        self.assertEqual(roll_parser.check_match('d4/'), False)

    def testBadLetter(self):
        self.assertEqual(roll_parser.check_match('4c4'), False)

    def testHangingD(self):
        self.assertEqual(roll_parser.check_match('4+4d'), False)

    def testBadParens(self):
        self.assertEqual(roll_parser.check_match('4(d4)'), False)

    def testJustParens(self):
        self.assertEqual(roll_parser.check_match('()'), False)

    def testParensAlone(self):
        self.assertEqual(roll_parser.check_match('d4+()'), False)


class TestCreateParensMap(unittest.TestCase):
    def testEmpty(self):
        self.assertEqual(roll_parser.create_parens_map(''), ('', {}))

    def testSinglePair(self):
        self.assertEqual(roll_parser.create_parens_map('()'), ('', {}))

    def testBigOleString(self):
        self.assertEqual(roll_parser.create_parens_map(
            '()a()()bbb(c)'), ('()a()()bbb(c)', {0: 1, 3: 4, 5: 6, 10: 12}))

    def testMultipleLayers(self):
        self.assertEqual(roll_parser.create_parens_map(
            '(()a()()bbb(()c))'), ('()a()()bbb(()c)', {0: 1, 3: 4, 5: 6, 11: 12, 10: 14}))


class TestFindSplitInd(unittest.TestCase):
    def testSimple(self):
        self.assertEqual(roll_parser.find_split_ind('3+4'), ('3+4', 1))

    def testSingleExpr(self):
        self.assertEqual(roll_parser.find_split_ind('4d4'), ('4d4', 2))

    def testLeftToRight(self):
        self.assertEqual(roll_parser.find_split_ind('4+3+2'), ('4+3+2', 3))

    def testAddAfterMult(self):
        self.assertEqual(roll_parser.find_split_ind('4+3*2'), ('4+3*2', 1))

    def testParens(self):
        self.assertEqual(roll_parser.find_split_ind('(4+3)*2'), ('(4+3)*2', 5))

    def testLeftParens(self):
        self.assertEqual(roll_parser.find_split_ind(
            '2*(4+3)*2'), ('2*(4+3)*2', 7))

    def testOrderOfOp(self):
        self.assertEqual(roll_parser.find_split_ind(
            '2+(4+3)*2'), ('2+(4+3)*2', 1))


# TODO
class TestParse(unittest.TestCase):
    def testMultiDiceOnly(self):
        seed(1234)
        self.assertEqual(roll_parser.parse('3d6'), 6)


if __name__ == "__main__":
    unittest.main()

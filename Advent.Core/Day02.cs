using System;
using System.Linq;

namespace Advent.Core
{
    public class Day02
    {
        public static void RunIntcodeProgram(int[] program)
        {
            int offset = 0;

            while (program[offset] != 99)
            {
                int opcode = program[offset];
                int operand1Index = program[offset + 1];
                int operand2Index = program[offset + 2];
                int resultIndex = program[offset + 3];

                int operand1 = program[operand1Index];
                int operand2 = program[operand2Index];

                int result = opcode switch
                {
                    1 => operand1 + operand2,
                    2 => operand1 * operand2,
                    _ => throw new InvalidOperationException("Invalid operator")
                };

                program[resultIndex] = result;
                offset += 4;
            }
        }
    }
}
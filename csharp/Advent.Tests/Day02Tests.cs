using System.Linq;
using Advent.Core;
using Advent.Core.Year2019;
using Xunit;

namespace Advent.Tests
{
    public class Day02Tests
    {
        [Theory]
        [InlineData("1,0,0,0,99", "2,0,0,0,99")]
        [InlineData("2,3,0,3,99", "2,3,0,6,99")]
        [InlineData("2,4,4,5,99,0", "2,4,4,5,99,9801")]
        [InlineData("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99")]
        public void RunIntcodeProgram(string code, string expectedCode)
        {
            var program = code.Split(",").Select(x => int.Parse(x)).ToArray();
            Day02.RunIntcodeProgram(program);
            var expected = expectedCode.Split(",").Select(x => int.Parse(x)).ToArray();
            Assert.Equal(expected.Length, program.Length);

            for (int i = 0; i < expected.Length; i++)
            {
                Assert.Equal(expected[i], program[i]);
            }
        }
    }
}
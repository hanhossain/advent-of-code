using System;
using Advent.Core;
using Xunit;

namespace Advent.Tests
{
    public class Day01Tests
    {
        [Theory]
        [InlineData(12, 2)]
        [InlineData(14, 2)]
        [InlineData(1969, 654)]
        [InlineData(100756, 33583)]
        public void GetFuelRequirements(int mass, int expectedFuel)
        {
            int fuel = Day01.GetFuelRequirements(mass);
            Assert.Equal(expectedFuel, fuel);
        }

        [Theory]
        [InlineData(14, 2)]
        [InlineData(1969, 966)]
        [InlineData(100756, 50346)]
        public void GetFuelRequirementsForFuel(int mass, int expectedFuel)
        {
            int fuel = Day01.GetFuelRequirementsForFuel(mass);
            Assert.Equal(expectedFuel, fuel);
        }
    }
}

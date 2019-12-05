using System;
using System.IO;
using System.Linq;
using Advent.Core;

namespace Advent.Runner
{
    class Program
    {
        static void Main(string[] args)
        {
            Day02Part1();
        }

        private static void Day01Part1()
        {
            var totalFuel = File.ReadLines("Data/Day01.txt").Select(x => Day01.GetFuelRequirements(int.Parse(x))).Sum();
            Console.WriteLine(totalFuel);
        }

        private static void Day01Part2()
        {
            var totalFuel = File.ReadLines("Data/Day01.txt").Select(x => Day01.GetFuelRequirementsForFuel(int.Parse(x))).Sum();
            Console.WriteLine(totalFuel);
        }

        private static void Day02Part1()
        {
            var program = File.ReadAllText("Data/Day02.txt").Split(",").Select(x => int.Parse(x)).ToArray();
            program[1] = 12;
            program[2] = 2;
            Day02.RunIntcodeProgram(program);
            Console.WriteLine(program[0]);
        }
    }
}

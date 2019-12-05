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
            Day01Part2();
        }

        public static void Day01Part1()
        {
            var totalFuel = File.ReadLines("Data/Day01.txt").Select(x => Day01.GetFuelRequirements(int.Parse(x))).Sum();
            Console.WriteLine(totalFuel);
        }

        public static void Day01Part2()
        {
            var totalFuel = File.ReadLines("Data/Day01.txt").Select(x => Day01.GetFuelRequirementsForFuel(int.Parse(x))).Sum();
            Console.WriteLine(totalFuel);
        }
    }
}

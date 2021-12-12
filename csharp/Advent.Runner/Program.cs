using System;
using System.IO;
using System.Linq;
using Advent.Core;
using Advent.Core.Year2019;

namespace Advent.Runner
{
    class Program
    {
        static void Main(string[] args)
        {
            Day02Part2();
        }

        private static void Day01Part1()
        {
            var totalFuel = File.ReadLines("Data/Year2019/Day01.txt").Select(x => Day01.GetFuelRequirements(int.Parse(x))).Sum();
            Console.WriteLine(totalFuel);
        }

        private static void Day01Part2()
        {
            var totalFuel = File.ReadLines("Data/Year2019/Day01.txt").Select(x => Day01.GetFuelRequirementsForFuel(int.Parse(x))).Sum();
            Console.WriteLine(totalFuel);
        }

        private static void Day02Part1()
        {
            var program = File.ReadAllText("Data/Year2019/Day02.txt").Split(",").Select(x => int.Parse(x)).ToArray();
            program[1] = 12;
            program[2] = 2;
            Day02.RunIntcodeProgram(program);
            Console.WriteLine(program[0]);
        }

        private static void Day02Part2()
        {
            var code = File.ReadAllText("Data/Year2019/Day02.txt");
            
            for (int noun = 0; noun < 100; noun++)
            {
                for (int verb = 0; verb < 100; verb++)
                {
                    if (Day02.RunIntcodeProgram(code, noun, verb) == 19690720)
                    {
                        Console.WriteLine(100 * noun + verb);
                        return;
                    }
                }
            }
        }
    }
}

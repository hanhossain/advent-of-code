namespace Advent.Core.Year2019
{
    public class Day01
    {
        public static int GetFuelRequirements(int mass)
        {
            return mass / 3 - 2;
        }

        public static int GetFuelRequirementsForFuel(int mass)
        {
            int fuel = GetFuelRequirements(mass);

            if (fuel <= 0)
            {
                return 0;
            }

            return fuel + GetFuelRequirementsForFuel(fuel);
        }
    }
}

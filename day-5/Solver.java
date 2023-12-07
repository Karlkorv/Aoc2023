import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Scanner;
import java.util.Set;

/**
 * Solver
 */
public class Solver {
    ArrayList<String> input = new ArrayList<>();
    ArrayList<HashMap<Range, Long>> maps = new ArrayList<>();
    ArrayList<Long> seeds = new ArrayList<>();
    long result;

    class Range {
        long start;
        long end;

        boolean contains(long num) {
            return num >= start && num <= end;
        }

        boolean overlaps(Range range) {
            return contains(range.start) || contains(range.end) || range.contains(start) || range.contains(end);
        }

        Range subRange(Range target) {
            if (!overlaps(target)) {
                return null;
            }
            long newStart = Math.max(start, target.start);
            long newEnd = Math.min(end, target.end);
            return new Range(newStart, newEnd);
        }

        Range(long start, long end) {
            this.start = start;
            this.end = end;
        }

        public String toString() {
            return "(" + start + " " + end + ")";
        }
    }

    void readInput() {
        Scanner sc = new Scanner(System.in);
        while (sc.hasNextLine()) {
            input.add(sc.nextLine());
        }
        sc.close();
    }

    void parseInput() {
        String[] seedStrings = input.get(0).replaceAll("seeds: ", "").split(" ");

        for (int i = 0; i < seedStrings.length; i += 2) {
            long seed = Long.parseLong(seedStrings[i]);
            seeds.add(seed);
        }

        int start = 2; // Rad 1 är seeds, rad 2 är blank
        for (int i = 2; i < input.size(); i++) {
            if (input.get(i).isBlank()) {
                parseMap(input.subList(start, i));
                continue;
            } else if (input.get(i).contains(":")) {
                start = i + 1;
                continue;
            } else if (i == input.size() - 1) { // Edge case för sista raden
                parseMap(input.subList(start, i + 1));
            }
        }
    }

    void parseMap(List<String> mapStr) {
        HashMap<Range, Long> map = new HashMap<>();
        for (String string : mapStr) {
            String[] splitStr = string.split(" ");

            long dest = Long.parseLong(splitStr[0]);
            long src = Long.parseLong(splitStr[1]);
            long range = Long.parseLong(splitStr[2]);

            Range sourceRange = new Range(src, src + range - 1);

            map.put(sourceRange, dest);
        }
        maps.add(map);
    }

    void calculateResult() {
        long result = Long.MAX_VALUE;
        int curSeed = 0;
        for (Range seedRange : seeds) {
            System.out.println("Seed: " + seedRange);
            for (long i = seedRange.start; i <= seedRange.end; i++) {
                if (i % 1000000 == 0) {
                    System.out.println(i);
                }
                long returnValue = getSeedDestination(i);
                if (returnValue < result) {
                    result = returnValue;
                }
            }
        }
        this.result = result;
    }

    long getSeedDestination(long seed) {
        long curSeed = seed;
        for (HashMap<Range, Long> map : maps) {
            for (Range range : map.keySet()) {
                if (range.contains(curSeed)) {
                    curSeed = map.get(range) + (curSeed - range.start);
                    break;
                }
            }
        }
        return curSeed;
    }

    Solver() {
        readInput();
        parseInput();
        calculateResult();
        printResult();
    }

    void printResult() {
        System.out.println(result);
    }

    public static void main(String[] args) {
        new Solver();
    }
}
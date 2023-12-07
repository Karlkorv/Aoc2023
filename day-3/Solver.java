import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

/**
 * Solver
 */
public class Solver {
    List<String> input = new ArrayList<>();
    List<Integer> results = new ArrayList<>();
    int result;

    class Gear {
        int x, y;
        ArrayList<Integer> adjacentNumbers = new ArrayList<>();
    }

    private void readInput() {
        Scanner sc = new Scanner(System.in);
        while (sc.hasNextLine()) {
            input.add(sc.nextLine());
        }
        sc.close();
    }

    private void parseInput() {
        parseGears();
    }

    private void parseGears() {
        Pattern gearPattern = Pattern.compile("\\*");
        Pattern nrPattern = Pattern.compile("(\\d+)");

        ArrayList<Gear> gears = new ArrayList<>();

        for (int i = 0; i < input.size(); i++) {
            Matcher m = gearPattern.matcher(input.get(i));
            while (m.find()) {
                Gear gear = new Gear();
                gear.x = m.start();
                gear.y = i;
                gears.add(gear);
            }
        }

        for (int i = 0; i < input.size(); i++) {
            String line = input.get(i);

            Matcher nrM = nrPattern.matcher(line);

            while (nrM.find()) {
                int nrStart = nrM.start();
                int nrEnd = nrM.end();

                for (Gear gear : gears) {
                    if (i < gear.y - 1 || i > gear.y + 1) {
                        continue;
                    }
                    if (gear.x >= nrStart - 1 && gear.x <= nrEnd) {
                        gear.adjacentNumbers.add(Integer.parseInt(nrM.group(1)));
                    }
                }
            }
        }

        for (Gear gear : gears) {
            if (gear.adjacentNumbers.size() != 2) {
                continue;
            }
            results.add(gear.adjacentNumbers.get(0) * gear.adjacentNumbers.get(1));
        }
    }

    private void parseVerticals() {
        Pattern nr = Pattern.compile("(\\d+)");
        Pattern symbols = Pattern.compile("[*\\/$=%@\\-&+#]");
        String topLine, line, bottomLine;

        for (int i = 0; i < input.size(); i++) {
            line = input.get(i);
            bottomLine = i + 1 < input.size() ? input.get(i + 1) : "";
            topLine = i - 1 >= 0 ? input.get(i - 1) : "";

            ArrayList<Integer> symbolIndices = new ArrayList<>();
            Matcher symbolsM = symbols.matcher(topLine);
            Matcher symbolsM2 = symbols.matcher(bottomLine);
            Matcher symbolsM3 = symbols.matcher(line);

            while (symbolsM.find()) {
                symbolIndices.add(symbolsM.start());
            }

            while (symbolsM2.find()) {
                symbolIndices.add(symbolsM2.start());
            }

            while (symbolsM3.find()) {
                symbolIndices.add(symbolsM3.start());
            }

            Matcher nrM = nr.matcher(line);

            while (nrM.find()) {
                int nrStart = nrM.start();
                int nrEnd = nrM.end();

                for (Integer integer : symbolIndices) {
                    if (nrStart - 1 <= integer && integer <= nrEnd) {
                        results.add(Integer.parseInt(nrM.group(1)));
                        break;
                    }
                }
            }
        }
    }

    private void calculateResult() {
        int sum = 0;
        for (Integer integer : results) {
            sum += integer;
        }
        result = sum;
    }

    private void printResult() {
        System.out.println(result);
    }

    public Solver() {
        readInput();
        parseInput();
        calculateResult();
        printResult();
    }

    public static void main(String[] args) {
        new Solver();
    }
}
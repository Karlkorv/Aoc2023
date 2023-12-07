import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import javax.management.RuntimeErrorException;

public class Solver {
    List<String> input;
    List<Integer> parsedInput;
    int result;

    public Solver() {
        readInput();
        parseInput();
        calcResult();
        printOutput();
    }

    private void readInput() {
        input = new ArrayList<>();
        Scanner scanner = new Scanner(System.in);
        while (scanner.hasNext()) {
            input.add(scanner.nextLine());
        }
        scanner.close();
    }

    private void parseInput1() {
        parsedInput = new ArrayList<>();
        Pattern pattern = Pattern.compile("(?:\\d.*\\d)|\\d");
        for (String string : input) {
            Matcher matcher = pattern.matcher(string);
            if (!matcher.find()) {
                System.out.println(string);
                throw new RuntimeException("hjälp");
            }
            String toTrim = matcher.group(0);
            char firstChar = toTrim.charAt(0);
            char secondChar = toTrim.charAt(toTrim.length() - 1);

            int out = (firstChar - 48) * 10 + secondChar - 48;
            parsedInput.add(out);
        }
    }

    private void parseInput() {
        parsedInput = new ArrayList<>();
        Pattern pattern = Pattern.compile(
                "(\\d|one|two|three|four|five|six|seven|eight|nine).*(\\d|one|two|three|four|five|six|seven|eight|nine)|\\d|(one|two|three|four|five|six|seven|eight|nine)");
        for (String string : input) {
            Matcher matcher = pattern.matcher(string);
            if (!matcher.find()) {
                System.out.println(string);
                throw new RuntimeException("hjälp");
            }
            if (matcher.group().length() == 1) {
                int firstChar = parse(matcher.group());
                parsedInput.add(firstChar * 10 + firstChar);
                continue;
            }
            String firstMatch = matcher.group(1);
            String lastMatch = matcher.group(2);
            int firstChar = parse(firstMatch);
            int secondChar = parse(lastMatch);
            int out = firstChar * 10 + secondChar;
            parsedInput.add(out);
        }
    }

    private int parse(String in) {
        switch (in) {
            case "one":
                return 1;
            case "two":
                return 2;
            case "three":
                return 3;
            case "four":
                return 4;
            case "five":
                return 5;
            case "six":
                return 6;
            case "seven":
                return 7;
            case "eight":
                return 8;
            case "nine":
                return 9;
            default:
                return Integer.parseInt(in);
        }
    }

    private void calcResult() {
        int sum = 0;
        for (Integer nr : parsedInput) {
            sum += nr;
        }
        result = sum;
    }

    private void printOutput() {
        System.out.println(result);
    }

    public static void main(String[] args) {
        new Solver();
    }
}
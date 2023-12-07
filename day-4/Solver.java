import java.util.ArrayList;
import java.util.Scanner;

/**
 * Solver
 */
public class Solver {
    ArrayList<String> input = new ArrayList<>();
    ArrayList<Card> cards = new ArrayList<>();
    int result;

    /**
     * Card
     */
    class Card {
        int[] winningNumbers;
        int[] actualNumbers;

        int points() {
            int points = 0;
            for (int nr : winningNumbers) {
                for (int nr2 : actualNumbers) {
                    if (nr == nr2) {
                        points = points == 0 ? 1 : points * 2;
                    }
                }
            }
            return points;
        }

        int matches() {
            int matches = 0;
            for (int nr : winningNumbers) {
                for (int nr2 : actualNumbers) {
                    if (nr == nr2) {
                        matches++;
                    }
                }
            }
            return matches;
        }

        public Card(String[] winning, String[] actual) {
            winningNumbers = new int[winning.length];
            actualNumbers = new int[actual.length];

            for (int i = 0; i < winning.length; i++) {
                winningNumbers[i] = Integer.parseInt(winning[i]);
            }

            for (int i = 0; i < actual.length; i++) {
                actualNumbers[i] = Integer.parseInt(actual[i]);
            }
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
        for (int i = 0; i < input.size(); i++) {
            String[] numbers = input.get(i).split("\\|");
            numbers[0] = numbers[0].replaceAll("Card\\s+\\d+:", "");
            Card card = new Card(numbers[0].trim().split("\\s+"), numbers[1].trim().split("\\s+"));
            cards.add(card);
        }
    }

    void calculateResult() {
        int[] matches = new int[cards.size()];

        for (int i = 0; i < matches.length; i++) {
            matches[i] = cards.get(i).matches();
        }

        result = recursiveMatches(matches);
    }

    int recursiveMatches(int[] matches) {
        int total = 0;

        for (int i = 0; i < matches.length; i++) {
            total += recursiveMatches(matches, i);
        }

        total += matches.length;

        return total;
    }

    int recursiveMatches(int[] matches, int index) {
        if (index >= matches.length || matches[index] == 0) {
            return 0;
        }

        int total = matches[index];

        for (int i = index + 1; i <= index + matches[index]; i++) {
            total += recursiveMatches(matches, i);
        }

        return total;
    }

    void printResult() {
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
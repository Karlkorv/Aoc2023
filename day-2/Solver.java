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
    List<Game> games = new ArrayList<>();
    Pattern gameId = Pattern.compile("Game (\\d+):");
    Pattern red = Pattern.compile("(\\d+) red");
    Pattern green = Pattern.compile("(\\d+) green");
    Pattern blue = Pattern.compile("(\\d+) blue");

    int result;

    final int MAX_RED = 12;
    final int MAX_GREEN = 13;
    final int MAX_BLUE = 14;

    class Game {
        int id;
        int maxRed;
        int maxBlue;
        int maxGreen;
    }

    public Solver() {
        readInput();
        parseInput();
        calculateResult();
        printResult();
    }

    private void readInput() {
        Scanner sc = new Scanner(System.in);
        while (sc.hasNextLine()) {
            input.add(sc.nextLine());
        }
        sc.close();
    }

    private void parseInput() {
        for (String input : input) {
            games.add(parseGame(input));
        }
    }

    private Game parseGame(String input) {
        Game game = new Game();
        Matcher matcher = gameId.matcher(input);
        if (matcher.find()) {
            game.id = Integer.parseInt(matcher.group(1));
        }
        String[] split = input.split(";");
        for (String string : split) {
            Matcher redMatcher = red.matcher(string);
            if (redMatcher.find()) {
                int redValue = Integer.parseInt(redMatcher.group(1));
                if (redValue > game.maxRed) {
                    game.maxRed = redValue;
                }
            }

            Matcher greenMatcher = green.matcher(string);
            if (greenMatcher.find()) {
                int greenValue = Integer.parseInt(greenMatcher.group(1));
                if (greenValue > game.maxGreen) {
                    game.maxGreen = greenValue;
                }
            }

            Matcher blueMatcher = blue.matcher(string);
            if (blueMatcher.find()) {
                int blueValue = Integer.parseInt(blueMatcher.group(1));
                if (blueValue > game.maxBlue) {
                    game.maxBlue = blueValue;
                }
            }
        }
        return game;
    }

    private void calculateResult() {
        int sum = 0;
        for (Game game : games) {
            sum += game.maxBlue * game.maxRed * game.maxGreen;
        }
        result = sum;
    }

    private void printResult() {
        System.out.println(result);
    }

    public static void main(String[] args) {
        new Solver();
    }
}